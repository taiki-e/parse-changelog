#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::default_trait_access, clippy::wildcard_imports)]

use std::{
    fs,
    io::{self, Read, Write},
};

use anyhow::{bail, Context as _, Result};
use clap::{AppSettings, ArgSettings, Clap};
use parse_changelog::Parser;

const ABOUT: &str = "Simple changelog parser, written in Rust.

Parses changelog and returns a release note for the specified version.

Use -h for short descriptions and --help for more details.";

const MAX_TERM_WIDTH: usize = 100;

#[derive(Clap)]
#[clap(
    about(ABOUT),
    version,
    max_term_width(MAX_TERM_WIDTH),
    setting(AppSettings::DeriveDisplayOrder),
    setting(AppSettings::UnifiedHelpMessage)
)]
struct Args {
    /// Path to the changelog file (use '-' for standard input).
    #[clap(value_name = "PATH", setting(ArgSettings::ForbidEmptyValues))]
    path: String,
    /// Specify version (by default, select the latest release).
    #[clap(value_name = "VERSION", setting(ArgSettings::ForbidEmptyValues))]
    release: Option<String>,
    /// Returns title instead of notes.
    #[clap(short, long)]
    title: bool,
    /// Returns JSON representation of all releases in changelog.
    #[clap(long, conflicts_with = "version", conflicts_with = "title")]
    json: bool,
    /// Specify version format.
    #[clap(long, value_name = "PATTERN", setting(ArgSettings::ForbidEmptyValues))]
    version_format: Option<String>,
    /// Specify prefix format.
    ///
    /// By default only "v", "Version ", "Release ", and "" (no prefix) are
    /// allowed as prefixes.
    #[clap(long, value_name = "PATTERN", visible_alias = "prefix")]
    prefix_format: Option<String>,
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {:#}", e);
        std::process::exit(1)
    }
}

fn try_main() -> Result<()> {
    let args = Args::parse();

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
    stdout.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::Path, process::Command};

    use anyhow::Result;
    use clap::IntoApp;
    use tempfile::Builder;

    use super::{Args, MAX_TERM_WIDTH};

    fn get_help(long: bool) -> Result<String> {
        let mut buf = vec![];
        if long {
            Args::into_app().term_width(MAX_TERM_WIDTH).write_long_help(&mut buf)?;
        } else {
            Args::into_app().term_width(MAX_TERM_WIDTH).write_help(&mut buf)?;
        }
        let mut out = String::new();
        for mut line in String::from_utf8(buf)?.lines() {
            if let Some(new) = line.trim_end().strip_suffix(env!("CARGO_PKG_VERSION")) {
                line = new;
            }
            out.push_str(line.trim_end());
            out.push('\n');
        }
        Ok(out)
    }

    #[track_caller]
    fn assert_diff(expected_path: impl AsRef<Path>, actual: impl AsRef<str>) {
        let actual = actual.as_ref();
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let expected_path = &manifest_dir.join(expected_path);
        if !expected_path.is_file() {
            fs::write(expected_path, "").unwrap();
        }
        let expected = fs::read_to_string(expected_path).unwrap();
        if expected != actual {
            if env::var_os("CI").is_some() {
                let outdir = Builder::new().prefix("assert_diff").tempdir().unwrap();
                let actual_path = &outdir.path().join(expected_path.file_name().unwrap());
                fs::write(actual_path, actual).unwrap();
                let status = Command::new("git")
                    .args(&["--no-pager", "diff", "--no-index", "--"])
                    .args(&[expected_path, actual_path])
                    .status()
                    .unwrap();
                assert!(!status.success());
                panic!("assertion failed");
            } else {
                fs::write(expected_path, actual).unwrap();
            }
        }
    }

    #[test]
    fn long_help() {
        let actual = get_help(true).unwrap();
        assert_diff("tests/long-help.txt", actual);
    }

    #[test]
    fn short_help() {
        let actual = get_help(false).unwrap();
        assert_diff("tests/short-help.txt", actual);
    }

    #[test]
    fn update_readme() -> Result<()> {
        let new = get_help(true)?;
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
                out.push_str(&new);
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
