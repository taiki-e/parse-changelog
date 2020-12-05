#![warn(rust_2018_idioms, single_use_lifetimes)]

mod auxiliary;

use auxiliary::{assert_diff, trim};
use parse_changelog::{parse, Parser};

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
        assert_eq!(trim(&changelog[0].notes), "0.2.0.");

        assert_eq!(changelog["0.2.0"].title, "0.2.0");
        assert_eq!(trim(&changelog["0.2.0"].notes), "0.2.0.");

        assert_eq!(changelog["0.1.0"].title, "0.1.0");
        assert_eq!(trim(&changelog["0.1.0"].notes), "0.1.0.");
    }
}

#[test]
fn failure() {
    let changelogs = [
        // Atx-style & 4 indents
        "    ## 0.1.0\n    0.1.0.\n",
        // Setext-style & 4 indents
        "    0.1.0\n    ==\n    0.1.0.\n",
    ];

    for changelog in &changelogs {
        assert!(parse(changelog).is_err());
    }
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
    let changelog = parse(include_str!("fixtures/rust.md")).unwrap();
    assert_diff("tests/fixtures/rust-1.46.0.md", &changelog["1.46.0"].notes);
}

#[test]
fn pin_project() {
    let changelog = parse(include_str!("fixtures/pin-project.md")).unwrap();
    assert_diff("tests/fixtures/pin-project-1.0.0.md", &changelog["1.0.0"].notes);
}

#[test]
fn cargo() {
    let changelog = Parser::new()
        .prefix_format("Cargo ")
        .unwrap()
        .version_format(r"^\d+\.\d+")
        .unwrap()
        .parse(include_str!("fixtures/cargo.md"))
        .unwrap();
    assert_diff("tests/fixtures/cargo-1.50.md", &changelog["1.50"].notes);
}
