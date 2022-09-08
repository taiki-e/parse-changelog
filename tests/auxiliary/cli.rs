use std::{
    env,
    ffi::OsStr,
    process::{Command, ExitStatus},
};

use anyhow::Context as _;
use easy_ext::ext;

pub fn parse_changelog<O: AsRef<OsStr>>(args: impl AsRef<[O]>) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_parse-changelog"));
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd.args(args.as_ref());
    cmd
}

#[ext(CommandExt)]
impl Command {
    #[track_caller]
    pub fn assert_output(&mut self) -> AssertOutput {
        let output = self.output().context("could not execute process").unwrap();
        AssertOutput {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            status: output.status,
        }
    }

    #[track_caller]
    pub fn assert_success(&mut self) -> AssertOutput {
        let output = self.assert_output();
        if !output.status.success() {
            panic!(
                "assertion failed: `self.status.success()`:\n\nSTDOUT:\n{0}\n{1}\n{0}\n\nSTDERR:\n{0}\n{2}\n{0}\n",
                "-".repeat(60),
                output.stdout,
                output.stderr,
            );
        }
        output
    }

    #[track_caller]
    pub fn assert_failure(&mut self) -> AssertOutput {
        let output = self.assert_output();
        if output.status.success() {
            panic!(
                "assertion failed: `!self.status.success()`:\n\nSTDOUT:\n{0}\n{1}\n{0}\n\nSTDERR:\n{0}\n{2}\n{0}\n",
                "-".repeat(60),
                output.stdout,
                output.stderr,
            );
        }
        output
    }
}

pub struct AssertOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}

fn line_separated(lines: &str) -> impl Iterator<Item = &'_ str> {
    lines.split('\n').map(str::trim).filter(|line| !line.is_empty())
}

impl AssertOutput {
    /// Receives a line(`\n`)-separated list of patterns and asserts whether stderr contains each pattern.
    #[track_caller]
    pub fn stderr_contains(&self, pats: &str) -> &Self {
        for pat in line_separated(pats) {
            if !self.stderr.contains(pat) {
                panic!(
                    "assertion failed: `self.stderr.contains(..)`:\n\nEXPECTED:\n{0}\n{pat}\n{0}\n\nACTUAL:\n{0}\n{1}\n{0}\n",
                    "-".repeat(60),
                    self.stderr
                );
            }
        }
        self
    }

    /// Receives a line(`\n`)-separated list of patterns and asserts whether stdout contains each pattern.
    #[track_caller]
    pub fn stderr_not_contains(&self, pats: &str) -> &Self {
        for pat in line_separated(pats) {
            if self.stderr.contains(pat) {
                panic!(
                    "assertion failed: `!self.stderr.contains(..)`:\n\nEXPECTED:\n{0}\n{pat}\n{0}\n\nACTUAL:\n{0}\n{1}\n{0}\n",
                    "-".repeat(60),
                    self.stderr
                );
            }
        }
        self
    }

    #[track_caller]
    pub fn stdout_eq(&self, s: &str) -> &Self {
        assert_eq!(self.stdout.trim(), s.trim());
        self
    }

    /// Receives a line(`\n`)-separated list of patterns and asserts whether stdout contains each pattern.
    #[track_caller]
    pub fn stdout_contains(&self, pats: &str) -> &Self {
        for pat in line_separated(pats) {
            if !self.stdout.contains(pat) {
                panic!(
                    "assertion failed: `self.stdout.contains(..)`:\n\nEXPECTED:\n{0}\n{pat}\n{0}\n\nACTUAL:\n{0}\n{1}\n{0}\n",
                    "-".repeat(60),
                    self.stdout
                );
            }
        }
        self
    }

    /// Receives a line(`\n`)-separated list of patterns and asserts whether stdout contains each pattern.
    #[track_caller]
    pub fn stdout_not_contains(&self, pats: &str) -> &Self {
        for pat in line_separated(pats) {
            if self.stdout.contains(pat) {
                panic!(
                    "assertion failed: `!self.stdout.contains(..)`:\n\nEXPECTED:\n{0}\n{pat}\n{0}\n\nACTUAL:\n{0}\n{1}\n{0}\n",
                    "-".repeat(60),
                    self.stdout
                );
            }
        }
        self
    }
}
