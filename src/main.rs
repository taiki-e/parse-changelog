#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::default_trait_access, clippy::wildcard_imports)]

use std::{
    fs,
    io::{self, Read, Write},
};

use anyhow::{bail, Context as _, Result};
use lexopt::{
    Arg::{Long, Short, Value},
    ValueExt,
};
use parse_changelog::Parser;

static USAGE: &str = "parse-changelog

Simple changelog parser, written in Rust.

Parses changelog and returns a release note for the specified version.

USAGE:
    parse-changelog [OPTIONS] <PATH> [VERSION]

ARGS:
    <PATH>       Path to the changelog file (use '-' for standard input)
    [VERSION]    Specify version (by default, select the latest release)

OPTIONS:
    -t, --title                       Returns title instead of notes
        --json                        Returns JSON representation of all releases in changelog
        --version-format <PATTERN>    Specify version format
        --prefix-format <PATTERN>     Specify prefix format [aliases: prefix]
    -h, --help                        Print help information
    -V, --version                     Print version information
";

struct Args {
    path: String,
    release: Option<String>,
    title: bool,
    json: bool,
    version_format: Option<String>,
    prefix_format: Option<String>,
}

impl Args {
    fn parse() -> Result<Self> {
        let mut path = None;
        let mut release = None;
        let mut title = false;
        let mut json = false;
        let mut version_format = None;
        let mut prefix_format = None;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('t') | Long("title") if !title => title = true,
                Long("json") if !json => json = true,
                Long("version-format") if version_format.is_none() => {
                    version_format = Some(parser.value()?.parse()?);
                }
                Long("prefix-format") | Long("prefix") if prefix_format.is_none() => {
                    prefix_format = Some(parser.value()?.parse()?);
                }
                Short('h') | Long("help") => {
                    print!("{}", USAGE);
                    std::process::exit(0);
                }
                Short('V') | Long("version") => {
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                Value(val) if path.is_none() => path = Some(val.parse()?),
                Value(val) if release.is_none() => release = Some(val.parse()?),
                _ => return Err(arg.unexpected().into()),
            }
        }

        let path = match path {
            Some(path) => path,
            None => bail!("no changelog path specified"),
        };

        Ok(Self { path, release, title, json, version_format, prefix_format })
    }
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {:#}", e);
        std::process::exit(1)
    }
}

fn try_main() -> Result<()> {
    let args = Args::parse()?;

    let mut parser = Parser::new();
    if let Some(version_format) = &args.version_format {
        parser.version_format(version_format)?;
    }
    if let Some(prefix_format) = &args.prefix_format {
        parser.prefix_format(prefix_format)?;
    }

    let text = if args.path == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).context("failed to read standard input")?;
        buf
    } else {
        fs::read_to_string(&args.path).with_context(|| format!("failed to read {}", args.path))?
    };

    let changelog = parser.parse(&text)?;

    if args.json {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        serde_json::to_writer(&mut stdout, &changelog)?;
        stdout.flush()?;
        return Ok(());
    }

    let release = if let Some(version) = args.release.as_deref() {
        if let Some(release) = changelog.get(version) {
            release
        } else {
            bail!("not found release note for '{}'", version);
        }
    } else {
        let (entry_key, entry_value) = changelog.first().context("not found release")?;

        if entry_key != &"Unreleased" {
            entry_value
        } else {
            (
                changelog.get_index(1).context(
                    format!("not found release; to get 'Unreleased' section specify release explicitly: `parse-changelog {} Unreleased`", args.path)
                )?
            ).1
        }
    };
    let text = if args.title { release.title } else { release.notes };
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(text.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        io::Write,
        path::Path,
        process::{Command, Stdio},
    };

    use anyhow::Result;

    use crate::USAGE;

    #[track_caller]
    fn assert_diff(expected_path: impl AsRef<Path>, actual: impl AsRef<str>) {
        let actual = actual.as_ref();
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let manifest_dir =
            manifest_dir.strip_prefix(env::current_dir().unwrap()).unwrap_or(manifest_dir);
        let expected_path = &manifest_dir.join(expected_path);
        if !expected_path.is_file() {
            fs::write(expected_path, "").unwrap();
        }
        let expected = fs::read_to_string(expected_path).unwrap();
        if expected != actual {
            if env::var_os("CI").is_some() {
                let mut child = Command::new("git")
                    .args(["--no-pager", "diff", "--no-index", "--"])
                    .arg(expected_path)
                    .arg("-")
                    .stdin(Stdio::piped())
                    .spawn()
                    .unwrap();
                child.stdin.as_mut().unwrap().write_all(actual.as_bytes()).unwrap();
                assert!(!child.wait().unwrap().success());
                // patch -p1 <<'EOF' ... EOF
                panic!("assertion failed; please run test locally and commit resulting changes, or apply above diff as patch");
            } else {
                fs::write(expected_path, actual).unwrap();
            }
        }
    }

    #[test]
    fn update_readme() -> Result<()> {
        let new = USAGE;
        let path = &Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md");
        let base = fs::read_to_string(path)?;
        let mut out = String::with_capacity(base.capacity());
        let mut lines = base.lines();
        let mut start = false;
        let mut end = false;
        while let Some(line) = lines.next() {
            out.push_str(line);
            out.push('\n');
            if line == "<!-- readme-long-help:start -->" {
                start = true;
                out.push_str("```console\n");
                out.push_str("$ parse-changelog --help\n");
                out.push_str(new);
                for line in &mut lines {
                    if line == "<!-- readme-long-help:end -->" {
                        out.push_str("```\n");
                        out.push_str(line);
                        out.push('\n');
                        end = true;
                        break;
                    }
                }
            }
        }
        if start && end {
            assert_diff(path, out);
        } else if start {
            panic!("missing `<!-- readme-long-help:end -->` comment in README.md");
        } else {
            panic!("missing `<!-- readme-long-help:start -->` comment in README.md");
        }
        Ok(())
    }
}
