// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(dead_code, unused_macros)]

#[cfg(feature = "default")]
pub(crate) mod cli;

use std::{
    env,
    io::Write as _,
    path::Path,
    process::{Command, Stdio},
    str,
};

use fs_err as fs;

pub(crate) fn trim(s: &str) -> &str {
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
pub(crate) fn assert_diff(expected_path: impl AsRef<Path>, actual: impl AsRef<[u8]>) {
    let actual = actual.as_ref();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected_path = &manifest_dir.join(expected_path);
    if !expected_path.is_file() {
        fs::create_dir_all(expected_path.parent().unwrap()).unwrap();
        fs::write(expected_path, "").unwrap();
    }
    let expected = fs::read(expected_path).unwrap();
    // TODO: remove trim_ascii
    if expected.trim_ascii() != actual.trim_ascii() {
        if env::var_os("CI").is_some() {
            let mut child = Command::new("git")
                .args(["--no-pager", "diff", "--no-index", "--"])
                .arg(expected_path)
                .arg("-")
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();
            child.stdin.as_mut().unwrap().write_all(actual).unwrap();
            assert!(!child.wait().unwrap().success());
            // patch -p1 <<'EOF' ... EOF
            panic!("assertion failed; please run test locally and commit resulting changes, or apply above diff as patch");
        } else {
            fs::write(expected_path, actual).unwrap();
        }
    }
}
