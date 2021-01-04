#![forbid(unsafe_code)]
#![warn(future_incompatible, rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all, clippy::default_trait_access)]

use anyhow::{bail, Context as _, Result};
use parse_changelog::Parser;
use std::{
    fs,
    io::{self, Read},
};
use structopt::{clap::AppSettings, StructOpt};

/// Parses changelog and returns a release note for the specified version.
#[derive(StructOpt)]
#[structopt(
    setting = AppSettings::UnifiedHelpMessage,
    setting = AppSettings::DeriveDisplayOrder,
    rename_all = "kebab-case",
)]
struct Args {
    /// Path to the changelog file (use '-' for standard input).
    #[structopt(value_name = "PATH")]
    path: String,
    /// Specify version (by default, select the latest release).
    #[structopt(value_name = "VERSION")]
    version: Option<String>,
    /// Returns title instead of notes.
    #[structopt(short, long)]
    title: bool,
    /// Specify version format.
    #[structopt(long, value_name = "PATTERN")]
    version_format: Option<String>,
    /// Alias for --prefix-format.
    #[structopt(long, value_name = "PATTERN")]
    prefix: Option<String>,
    /// Specify prefix format.
    #[structopt(long, value_name = "PATTERN", conflicts_with = "prefix")]
    prefix_format: Option<String>,
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {:#}", e);
        std::process::exit(1)
    }
}

fn try_main() -> Result<()> {
    let args = Args::from_args();

    let mut parser = Parser::new();
    if let Some(version_format) = &args.version_format {
        parser.version_format(version_format)?;
    }
    if let Some(prefix_format) = args.prefix_format.as_ref().map_or(args.prefix.as_ref(), Some) {
        parser.prefix_format(prefix_format)?;
    }

    let text = if args.path == "-" {
        let mut buf = String::new();
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buf).context("failed to read standard input")?;
        buf
    } else {
        fs::read_to_string(&args.path).with_context(|| format!("failed to read {}", args.path))?
    };

    let changelog = parser.parse(&text)?;
    let release = if let Some(version) = args.version.as_deref() {
        if let Some(release) = changelog.get(version) {
            release
        } else {
            bail!("not found release note for '{}'", version);
        }
    } else {
        &changelog[0]
    };
    if args.title {
        println!("{}", release.title);
    } else {
        println!("{}", release.notes);
    }

    Ok(())
}
