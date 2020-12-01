#![allow(dead_code, unused_macros)]

use std::{fs, path::Path, process::Command, str};
use tempfile::Builder;

pub fn trim(s: &str) -> &str {
    let mut cnt = 0;
    while s[cnt..].starts_with(' ') {
        cnt += 1;
    }
    // Indents less than 4 are ignored.
    if cnt < 4 { s[cnt..].trim_end() } else { s.trim_end() }
}

pub fn assert_diff(expected_path: impl AsRef<Path>, actual: impl AsRef<str>) {
    let actual = actual.as_ref();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected_path = &manifest_dir.join(expected_path);
    (|| -> Result<(), Box<dyn std::error::Error>> {
        let expected = fs::read_to_string(expected_path)?;
        if expected != actual {
            let outdir = Builder::new().prefix("assert_diff").tempdir()?;
            let actual_path = &outdir.path().join(expected_path.file_name().unwrap());
            fs::write(actual_path, actual)?;
            let status = Command::new("git")
                .args(&["--no-pager", "diff", "--no-index", "--"])
                .args(&[expected_path, actual_path])
                .status()?;
            assert!(!status.success());
            panic!("assertion failed");
        }
        Ok(())
    })()
    .unwrap_or_else(|e| panic!("{}", e))
}
