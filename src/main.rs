#![forbid(unsafe_code)]
#![warn(future_incompatible, rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all, clippy::default_trait_access)]

use anyhow::{bail, Context as _};
use parse_changelog::Parser;
use std::fs;
use structopt::{clap::AppSettings, StructOpt};

type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

/// Parses changelog and returns a release note for the specified version.
#[derive(StructOpt)]
#[structopt(
    setting = AppSettings::UnifiedHelpMessage,
    setting = AppSettings::DeriveDisplayOrder,
    rename_all = "kebab-case",
)]
struct Args {
    /// Path to the changelog file.
    #[structopt(value_name = "PATH")]
    path: String,
    /// Specify version (by default, select the latest release).
    #[structopt(value_name = "VERSION")]
    version: Option<String>,
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

    let changelog =
        fs::read_to_string(&args.path).with_context(|| format!("failed to read {}", args.path))?;
    let mut parser = Parser::new();
    if let Some(version_format) = &args.version_format {
        parser.version_format(version_format)?;
    }
    if let Some(prefix_format) = args.prefix_format.as_ref().map_or(args.prefix.as_ref(), Some) {
        parser.prefix_format(prefix_format)?;
    }
    let changelog = parser.parse(&changelog)?;
    if let Some(version) = args.version.as_deref() {
        if let Some(release) = changelog.get(version) {
            println!("{}", release.notes);
        } else {
            bail!("not found release note for '{}'", version);
        }
    } else {
        println!("{}", changelog[0].notes);
    }

    Ok(())
}
