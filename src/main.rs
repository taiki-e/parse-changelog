// SPDX-License-Identifier: Apache-2.0 OR MIT

#![forbid(unsafe_code)]

use std::{
    fs,
    io::{self, Read as _, Write as _},
    path::{Path, PathBuf},
};

use lexopt::{
    Arg::{Long, Short, Value},
    ValueExt as _,
};
use parse_changelog::Parser;

type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

macro_rules! bail {
    ($($tt:tt)*) => {
        return Err(format!($($tt)*).into())
    };
}

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
        --title-no-link               Similar to --title, but remove links from title
        --json                        Returns JSON representation of all releases in changelog
        --version-format <PATTERN>    Specify version format
        --prefix-format <PATTERN>     Specify prefix format [aliases: prefix]
    -h, --help                        Print help information
    -V, --version                     Print version information
";

struct Args {
    path: PathBuf,
    release: Option<String>,
    title: bool,
    title_no_link: bool,
    json: bool,
    version_format: Option<String>,
    prefix_format: Option<String>,
}

impl Args {
    fn parse() -> Result<Self> {
        fn format_arg(arg: &lexopt::Arg<'_>) -> String {
            match arg {
                Long(flag) => format!("--{flag}"),
                Short(flag) => format!("-{flag}"),
                Value(val) => val.parse().unwrap(),
            }
        }
        #[cold]
        #[inline(never)]
        fn multi_arg(flag: &lexopt::Arg<'_>) -> Result<()> {
            let flag = &format_arg(flag);
            bail!("the argument '{flag}' was provided more than once, but cannot be used multiple times");
        }
        #[cold]
        #[inline(never)]
        fn conflicts(a: &str, b: &str) -> Result<()> {
            bail!("{a} may not be used together with {b}");
        }

        let mut path = None;
        let mut release = None;
        let mut title = false;
        let mut title_no_link = false;
        let mut json = false;
        let mut version_format = None;
        let mut prefix_format = None;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            macro_rules! parse_flag {
                ($flag:ident $(,)?) => {{
                    if std::mem::replace(&mut $flag, true) {
                        multi_arg(&arg)?;
                    }
                }};
            }
            macro_rules! parse_opt {
                ($opt:ident $(,)?) => {{
                    if $opt.is_some() {
                        multi_arg(&arg)?;
                    }
                    $opt = Some(parser.value()?.parse()?);
                }};
            }
            match arg {
                Short('t') | Long("title") => parse_flag!(title),
                Long("title-no-link") => parse_flag!(title_no_link),
                Long("json") => parse_flag!(json),
                Long("version-format") => parse_opt!(version_format),
                Long("prefix-format" | "prefix") => parse_opt!(prefix_format),
                Short('h') | Long("help") => {
                    print!("{USAGE}");
                    std::process::exit(0);
                }
                Short('V') | Long("version") => {
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                Value(val) if path.is_none() => path = Some(val.into()),
                Value(val) if release.is_none() => release = Some(val.parse()?),
                _ => return Err(arg.unexpected().into()),
            }
        }

        let Some(path) = path else { bail!("no changelog path specified") };
        if title && title_no_link {
            conflicts("--title", "--title-no-link")?;
        }

        Ok(Self { path, release, title, title_no_link, json, version_format, prefix_format })
    }

    fn path_for_msg(&self) -> &Path {
        if self.path.as_os_str() == "-" {
            Path::new("changelog (standard input)")
        } else {
            &self.path
        }
    }
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {e}");
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

    let text = if args.path.as_os_str() == "-" {
        let mut buf = String::with_capacity(128);
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("failed to read from standard input: {e}"))?;
        buf
    } else {
        fs::read_to_string(&args.path)
            .map_err(|e| format!("failed to read from file `{}`: {e}", args.path.display()))?
    };

    let changelog = match parser.parse(&text) {
        Ok(changelog) => changelog,
        Err(e) => {
            if e.is_parse() {
                bail!("{e} in {}", args.path_for_msg().display());
            }
            return Err(e.into());
        }
    };

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
            bail!("not found release note for '{version}' in {}", args.path_for_msg().display());
        }
    } else {
        let (entry_key, entry_value) = changelog.first().unwrap(); // unwrap is okay as Parser::parse returns an error if changelog is empty.
        if entry_key == &"Unreleased" {
            changelog
                .get_index(1)
                .ok_or_else(|| {
                    format!(
                        "not found release; to get 'Unreleased' section specify release \
                         explicitly: `parse-changelog {} Unreleased`",
                        args.path.display()
                    )
                })?
                .1
        } else {
            entry_value
        }
    };
    let text = if args.title {
        release.title.into()
    } else if args.title_no_link {
        release.title_no_link()
    } else {
        release.notes.into()
    };
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(text.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}
