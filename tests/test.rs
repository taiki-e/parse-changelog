#![warn(rust_2018_idioms, single_use_lifetimes)]

mod auxiliary;

use std::error::Error as _;

use auxiliary::{assert_diff, trim};
use parse_changelog::{parse, parse_iter, Parser};

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

    Parser::new().prefix_format("").unwrap();
    Parser::new().prefix_format("  ").unwrap();
    Parser::new().prefix_format("\t\n").unwrap();
    let e = Parser::new().prefix_format(r"\/").unwrap_err();
    assert!(e.is_format());
    assert!(e.source().is_some());

    assert!(Parser::new().version_format("").unwrap_err().is_format());
    assert!(Parser::new().version_format("  ").unwrap_err().is_format());
    assert!(Parser::new().version_format("\t\n").unwrap_err().is_format());
    let e = Parser::new().version_format(r"\/").unwrap_err();
    assert!(e.is_format());
    assert!(e.source().is_some());
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
fn rust() {
    let text = include_str!("fixtures/rust.md");
    let map = parse(text).unwrap();
    assert_eq!(map.len(), 72);
    assert_diff("tests/fixtures/rust-1.46.0.md", map["1.46.0"].notes);
    let vec: Vec<_> = parse_iter(text).collect();
    assert_eq!(vec.len(), 72);
    assert_eq!(vec[2], map["1.46.0"]);

    let text = include_str!("fixtures/rust-atx.md");
    let map = parse(text).unwrap();
    assert_eq!(map.len(), 72);
    assert_diff("tests/fixtures/rust-1.46.0-atx.md", map["1.46.0"].notes);
    let vec: Vec<_> = parse_iter(text).collect();
    assert_eq!(vec.len(), 72);
    assert_eq!(vec[2], map["1.46.0"]);
}

#[test]
fn pin_project() {
    let text = include_str!("fixtures/pin-project.md");
    let changelog = parse(text).unwrap();
    assert_eq!(changelog.len(), 70);
    assert_diff("tests/fixtures/pin-project-1.0.0.md", changelog["1.0.0"].notes);

    // empty prefix format
    let changelog = Parser::new().prefix_format("").unwrap().parse(text).unwrap();
    assert_eq!(changelog.len(), 70);
    assert_diff("tests/fixtures/pin-project-1.0.0.md", changelog["1.0.0"].notes);
}

#[test]
fn cargo() {
    let changelog = Parser::new()
        .prefix_format("Cargo ")
        .unwrap()
        .version_format(r"^[0-9]+\.[0-9]+")
        .unwrap()
        .parse(include_str!("fixtures/cargo.md"))
        .unwrap();
    assert_eq!(changelog.len(), 21);
    assert_diff("tests/fixtures/cargo-1.50.md", changelog["1.50"].notes);
}

// Regression tests for bugs caught by fuzzing.
#[test]
fn fuzz() {
    let tests =
        &[("1115.8.8 '9.\n-\n\u{c}\"----\u{19}\u{1f}<!--.4\n## 444.444.4\r\r \u{b}---->", Some(1))];
    for &(test, expected_len) in tests {
        let res = parse(test);
        if let Some(expected_len) = expected_len {
            assert_eq!(res.unwrap().len(), expected_len);
        }
    }
}
