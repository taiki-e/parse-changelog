#![allow(dead_code, unused_macros)]

#[cfg(feature = "default")]
mod cli;

use std::{
    env, fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    str,
};

#[cfg(feature = "default")]
pub use self::cli::*;

pub fn trim(s: &str) -> &str {
    let mut cnt = 0;
    while s[cnt..].starts_with(' ') {
        cnt += 1;
    }
    // Indents less than 4 are ignored.
    if cnt < 4 {
        s[cnt..].trim_end()
    } else {
        s.trim_end()
    }
}

#[track_caller]
pub fn assert_diff(expected_path: impl AsRef<Path>, actual: impl AsRef<str>) {
    let actual = actual.as_ref();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_dir =
        manifest_dir.strip_prefix(env::current_dir().unwrap()).unwrap_or(manifest_dir);
    let expected_path = &manifest_dir.join(expected_path);
    let expected = fs::read_to_string(expected_path).unwrap();
    if expected.trim() != actual.trim() {
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
    }
}
