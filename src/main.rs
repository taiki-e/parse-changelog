#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::default_trait_access, clippy::wildcard_imports)]

use std::{
    fs,
    io::{self, Read, Write},
};

use anyhow::{bail, Context as _, Result};
use parse_changelog::Parser;

static USAGE: &str = "parse-changelog

Simple changelog parser, written in Rust.

Parses changelog and returns a release note for the specified version.

Use -h for short descriptions and --help for more details.

USAGE:
    parse-changelog [OPTIONS] <PATH> [VERSION]

ARGS:
    <PATH>       Path to the changelog file (use '-' for standard input)
    <VERSION>    Specify version (by default, select the latest release)

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
    fn parse() -> Result<Args> {
        let mut args = pico_args::Arguments::from_env();

        if args.contains(["-h", "--help"]) {
            print!("{}", USAGE);
            std::process::exit(0);
        }
        if args.contains(["-V", "--version"]) {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }

        let this = Args {
            title: args.contains(["-t", "--title"]),
            json: args.contains("--json"),
            version_format: args.opt_value_from_str("--version-format")?,
            prefix_format: {
                let mut prefix_format = args.opt_value_from_str("--prefix-format")?;
                if prefix_format.is_none() {
                    prefix_format = args.opt_value_from_str("--prefix")?;
                }
                prefix_format
            },
            path: args.free_from_str()?,
            release: args.opt_free_from_str()?,
        };
        let remaining = args.finish();
        if !remaining.is_empty() {
            bail!("unrecognized arguments {:?}", remaining);
        }
        Ok(this)
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
        let mut stdout = io::stdout();
        serde_json::to_writer(stdout.lock(), &changelog)?;
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
        &changelog[0]
    };
    let text = if args.title { release.title } else { release.notes };
    let mut stdout = io::stdout();
    stdout.write_all(text.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::Path};

    use anyhow::Result;

    use crate::USAGE;

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
            fs::write(path, out)?;
        } else if start {
            panic!("missing `<!-- readme-long-help:end -->` comment in README.md");
        } else {
            panic!("missing `<!-- readme-long-help:start -->` comment in README.md");
        }
        Ok(())
    }
}
