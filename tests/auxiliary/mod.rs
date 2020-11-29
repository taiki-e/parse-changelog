use anyhow::Result;
use std::{fs, process::Command, str};
use tempfile::Builder;

pub fn trim(s: &str) -> &str {
    let mut cnt = 0;
    while s[cnt..].starts_with(' ') {
        cnt += 1;
    }
    // Indents less than 4 are ignored.
    if cnt < 4 { s[cnt..].trim_end() } else { s.trim_end() }
}

pub fn assert_eq(expected: &str, actual: &str) {
    (|| -> Result<()> {
        if expected != actual {
            let outdir = Builder::new().prefix("tests").tempdir()?;
            let expected_path = &outdir.path().join("expected.txt");
            let actual_path = &outdir.path().join("actual.txt");
            fs::write(expected_path, expected)?;
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
    .unwrap_or_else(|e| panic!("{:#}", e))
}
