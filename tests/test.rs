// SPDX-License-Identifier: Apache-2.0 OR MIT

mod auxiliary;

use parse_changelog::{parse, parse_iter, Parser};

use self::auxiliary::{assert_diff, trim};

#[test]
fn success() {
    let changelogs = [
        // Atx-style 1
        "
# Changelog
## unreleased
## 0.2.0
0.2.0.
## 0.1.0
0.1.0.
",
        // Atx-style 2
        "
# Changelog
# unreleased
# 0.2.0
0.2.0.
# 0.1.0
0.1.0.
",
        // Atx-style & 3 indents
        "
   # Changelog
   # unreleased
   # 0.2.0
   0.2.0.
   # 0.1.0
   0.1.0.
",
        // Setext-style 1
        "
Changelog
==
unreleased
--
0.2.0
--
0.2.0.
0.1.0
--
0.1.0.
",
        // Setext-style 2
        "
Changelog
==
unreleased
==
0.2.0
==
0.2.0.
0.1.0
==
0.1.0.
",
        // Setext-style & 3 indents
        "
   Changelog
   ==
   unreleased
   ==
   0.2.0
   ==
   0.2.0.
   0.1.0
   ==
   0.1.0.
",
    ];

    for changelog in &changelogs {
        let changelog = parse(changelog).unwrap();
        assert_eq!(changelog[0].title, "0.2.0");
        assert_eq!(trim(changelog[0].notes), "0.2.0.");

        assert_eq!(changelog["0.2.0"].title, "0.2.0");
        assert_eq!(trim(changelog["0.2.0"].notes), "0.2.0.");

        assert_eq!(changelog["0.1.0"].title, "0.1.0");
        assert_eq!(trim(changelog["0.1.0"].notes), "0.1.0.");
    }
}

#[test]
fn failure() {
    let changelogs = [
        // Atx-style & 4 indents
        "    ## 0.1.0\n",
        // Setext-style & 4 indents
        "    0.1.0\n    ==\n",
        // Setext-style & 4 indents
        "    0.1.0\n==\n",
        // Setext-style & 4 indents
        "    0.1.0\n--\n",
        // Setext-style & non-'=' char
        "0.1.0\n==?\n",
        // Setext-style & non-'-' char
        "0.1.0\n--?\n",
    ];
    for changelog in &changelogs {
        assert!(parse(changelog).unwrap_err().is_parse());
    }

    assert!(Parser::new().prefix_format("").is_ok());
    assert!(Parser::new().prefix_format("  ").is_ok());
    assert!(Parser::new().prefix_format("\t\n").is_ok());
    assert!(Parser::new().prefix_format(r"\/").is_ok());

    assert!(Parser::new().version_format("").unwrap_err().is_format());
    assert!(Parser::new().version_format("  ").unwrap_err().is_format());
    assert!(Parser::new().version_format("\t\n").unwrap_err().is_format());
    assert!(Parser::new().version_format(r"\/").is_ok());
}

#[test]
fn multiple_heading() {
    let changelogs = ["## 0.1.0\n## 0.1.0\n", "## 0.1.0\n## 0.1.0\n## 0.0.0\n"];
    for changelog in &changelogs {
        assert!(parse(changelog).unwrap_err().is_parse());
    }

    let changelogs = ["## 0.1.0\n##0.1.0\n", "##0.1.0\n## 0.1.0\n##0.0.0\n"];
    for changelog in &changelogs {
        assert_eq!(parse(changelog).unwrap().len(), 1);
    }
}

// Atx-style heading
#[test]
fn atx_heading() {
    for level in 1..=6 {
        let changelog = &format!("{} 0.1.0", "#".repeat(level));
        assert_eq!(1, parse(changelog).unwrap().len());
    }

    let changelog = &format!("{} 0.1.0", "#".repeat(7));
    assert!(parse(changelog).unwrap_err().is_parse());
}

#[test]
fn code_block() {
    let changelog = "\
# 0.2.0
```
# 0.2.0
```
# 0.1.0
```
# 0.1.0
```
";
    let changelog = parse(changelog).unwrap();
    assert_eq!(changelog.len(), 2);
    assert_eq!(changelog["0.2.0"].notes, "```\n# 0.2.0\n```");
    assert_eq!(changelog["0.1.0"].notes, "```\n# 0.1.0\n```");

    let changelog = "\
# 0.2.0
    # 0.2.0
# 0.1.0
    # 0.1.0
";
    let changelog = parse(changelog).unwrap();
    assert_eq!(changelog.len(), 2);
    assert_eq!(changelog["0.2.0"].notes, "    # 0.2.0");
    assert_eq!(changelog["0.1.0"].notes, "    # 0.1.0");
}

#[test]
fn comment() {
    let changelog = "\
# 0.2.0
<!--
# 0.2.0
-->
# 0.1.0
<!--
# 0.1.0
-->
";
    let changelog = parse(changelog).unwrap();
    assert_eq!(changelog.len(), 2);
    assert_eq!(changelog["0.2.0"].notes, "<!--\n# 0.2.0\n-->");
    assert_eq!(changelog["0.1.0"].notes, "<!--\n# 0.1.0\n-->");

    let changelog = "\
# 0.2.0
<!--
# 0.2.0-->
# 0.1.0
<!--
# 0.1.0-->
";
    let changelog = parse(changelog).unwrap();
    assert_eq!(changelog.len(), 2);
    assert_eq!(changelog["0.2.0"].notes, "<!--\n# 0.2.0-->");
    assert_eq!(changelog["0.1.0"].notes, "<!--\n# 0.1.0-->");

    let changelog = "\
# 0.2.0
<!--
# 0.2.0 --> <!--
# 0.2.0 -->
# 0.1.0
<!--
# 0.1.0 --> <!--
# 0.1.0 -->
# 0.0.0
";
    let changelog = parse(changelog).unwrap();
    assert_eq!(changelog.len(), 3);
    assert_eq!(changelog["0.2.0"].notes, "<!--\n# 0.2.0 --> <!--\n# 0.2.0 -->");
    assert_eq!(changelog["0.1.0"].notes, "<!--\n# 0.1.0 --> <!--\n# 0.1.0 -->");
    assert_eq!(changelog["0.0.0"].notes, "");
}

#[test]
fn link() {
    let changelog = "\
# [Version 0.5.2 2022-01-01]
# [Version 0.5.1 2022-01-01][link]
# [Version 0.5.0 2022-01-01](link)
# Version [0.4.2 2022-01-01]
# Version [0.4.1 2022-01-01][link]
# Version [0.4.0 2022-01-01](link)
# [0.3.2 2022-01-01]
# [0.3.1 2022-01-01][link]
# [0.3.0 2022-01-01](link)
# [0.2.2] 2022-01-01
# [0.2.1][link] 2022-01-01
# [0.2.0](link) 2022-01-01
# [0.1.2]
# [0.1.1][link]
# [0.1.0](link)
";
    let changelog = parse(changelog).unwrap();
    assert_eq!(changelog.len(), 15);
    assert_eq!(changelog["0.5.2"].version, "0.5.2");
    assert_eq!(changelog["0.5.2"].title, "[Version 0.5.2 2022-01-01]");
    assert_eq!(changelog["0.5.2"].title_no_link(), "Version 0.5.2 2022-01-01");
    assert_eq!(changelog["0.5.1"].version, "0.5.1");
    assert_eq!(changelog["0.5.1"].title, "[Version 0.5.1 2022-01-01][link]");
    assert_eq!(changelog["0.5.1"].title_no_link(), "Version 0.5.1 2022-01-01");
    assert_eq!(changelog["0.5.0"].version, "0.5.0");
    assert_eq!(changelog["0.5.0"].title, "[Version 0.5.0 2022-01-01](link)");
    assert_eq!(changelog["0.5.0"].title_no_link(), "Version 0.5.0 2022-01-01");
    assert_eq!(changelog["0.4.2"].version, "0.4.2");
    assert_eq!(changelog["0.4.2"].title, "Version [0.4.2 2022-01-01]");
    assert_eq!(changelog["0.4.2"].title_no_link(), "Version 0.4.2 2022-01-01");
    assert_eq!(changelog["0.4.1"].version, "0.4.1");
    assert_eq!(changelog["0.4.1"].title, "Version [0.4.1 2022-01-01][link]");
    assert_eq!(changelog["0.4.1"].title_no_link(), "Version 0.4.1 2022-01-01");
    assert_eq!(changelog["0.4.0"].version, "0.4.0");
    assert_eq!(changelog["0.4.0"].title, "Version [0.4.0 2022-01-01](link)");
    assert_eq!(changelog["0.4.0"].title_no_link(), "Version 0.4.0 2022-01-01");
    assert_eq!(changelog["0.3.2"].version, "0.3.2");
    assert_eq!(changelog["0.3.2"].title, "[0.3.2 2022-01-01]");
    assert_eq!(changelog["0.3.2"].title_no_link(), "0.3.2 2022-01-01");
    assert_eq!(changelog["0.3.1"].version, "0.3.1");
    assert_eq!(changelog["0.3.1"].title, "[0.3.1 2022-01-01][link]");
    assert_eq!(changelog["0.3.1"].title_no_link(), "0.3.1 2022-01-01");
    assert_eq!(changelog["0.3.0"].version, "0.3.0");
    assert_eq!(changelog["0.3.0"].title, "[0.3.0 2022-01-01](link)");
    assert_eq!(changelog["0.3.0"].title_no_link(), "0.3.0 2022-01-01");
    assert_eq!(changelog["0.2.2"].version, "0.2.2");
    assert_eq!(changelog["0.2.2"].title, "[0.2.2] 2022-01-01");
    assert_eq!(changelog["0.2.2"].title_no_link(), "0.2.2 2022-01-01");
    assert_eq!(changelog["0.2.1"].version, "0.2.1");
    assert_eq!(changelog["0.2.1"].title, "[0.2.1][link] 2022-01-01");
    assert_eq!(changelog["0.2.1"].title_no_link(), "0.2.1 2022-01-01");
    assert_eq!(changelog["0.2.0"].version, "0.2.0");
    assert_eq!(changelog["0.2.0"].title, "[0.2.0](link) 2022-01-01");
    assert_eq!(changelog["0.2.0"].title_no_link(), "0.2.0 2022-01-01");
    assert_eq!(changelog["0.1.2"].version, "0.1.2");
    assert_eq!(changelog["0.1.2"].title, "[0.1.2]");
    assert_eq!(changelog["0.1.2"].title_no_link(), "0.1.2");
    assert_eq!(changelog["0.1.1"].version, "0.1.1");
    assert_eq!(changelog["0.1.1"].title, "[0.1.1][link]");
    assert_eq!(changelog["0.1.1"].title_no_link(), "0.1.1");
    assert_eq!(changelog["0.1.0"].version, "0.1.0");
    assert_eq!(changelog["0.1.0"].title, "[0.1.0](link)");
    assert_eq!(changelog["0.1.0"].title_no_link(), "0.1.0");
}

#[test]
#[cfg_attr(miri, ignore)] // Miri is too slow
fn pin_project() {
    let text = include_str!("fixtures/pin-project.md");
    let changelog = parse(text).unwrap();
    assert_eq!(changelog.len(), 82);
    assert_diff("tests/fixtures/pin-project-1.0.0.md", changelog["1.0.0"].notes);

    // empty prefix format
    let changelog = Parser::new().prefix_format("").unwrap().parse(text).unwrap();
    assert_eq!(changelog.len(), 82);
    assert_diff("tests/fixtures/pin-project-1.0.0.md", changelog["1.0.0"].notes);
}
#[test]
#[cfg_attr(miri, ignore)] // Miri is too slow
fn rust() {
    let text = include_str!("fixtures/rust.md");
    let map = parse(text).unwrap();
    assert_eq!(map.len(), 116);
    assert_diff("tests/fixtures/rust-1.46.0.md", map["1.46.0"].notes);
    let vec: Vec<_> = parse_iter(text).collect();
    assert_eq!(vec.len(), map.len());
    assert_eq!(vec[46], map["1.46.0"]);

    let text = include_str!("fixtures/rust-atx.md");
    let map = parse(text).unwrap();
    assert_eq!(map.len(), 116);
    assert_diff("tests/fixtures/rust-1.46.0-atx.md", map["1.46.0"].notes);
    let vec: Vec<_> = parse_iter(text).collect();
    assert_eq!(vec.len(), map.len());
    assert_eq!(vec[46], map["1.46.0"]);
}
#[test]
#[cfg_attr(miri, ignore)] // Miri is too slow
fn cargo() {
    let changelog = Parser::new()
        .prefix_format("Cargo ")
        .unwrap()
        .version_format(r"^[0-9]+\.[0-9]+(\.[0-9])?$")
        .unwrap()
        .parse(include_str!("fixtures/cargo.md"))
        .unwrap();
    assert_eq!(changelog.len(), 54);
    assert_diff("tests/fixtures/cargo-1.50.md", changelog["1.50"].notes);
    assert_diff("tests/fixtures/cargo-1.77.1.md", changelog["1.77.1"].notes);
}

// Regression tests for bugs caught by fuzzing.
#[test]
fn fuzz() {
    let tests: &[(&str, Result<usize, &str>)] =
        &[("1115.8.8 '9.\n-\n\u{c}\"----\u{19}\u{1f}<!--.4\n## 444.444.4\r\r \u{b}---->", Ok(1))];
    for &(test, expected_len) in tests {
        let res = parse(test);
        match expected_len {
            Ok(expected_len) => {
                assert_eq!(res.unwrap().len(), expected_len);
            }
            Err(s) => {
                assert_eq!(res.unwrap_err().to_string(), s);
            }
        }
    }
}

#[test]
#[cfg_attr(miri, ignore)] // Miri is too slow
fn pathological_fake_heading() {
    let text = &"#".repeat(1024 * 1024 * 128);
    let now = std::time::Instant::now();
    parse(text).unwrap_err();
    eprintln!("{:?}", now.elapsed());
}
