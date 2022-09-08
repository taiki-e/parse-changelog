use super::*;

#[test]
fn test_extract_version_from_title() {
    fn version(s: &str) -> &str {
        extract_version_from_title(s, &DEFAULT_PREFIX_FORMAT)
    }

    assert_eq!(version("[1.0.0]"), "1.0.0");
    assert_eq!(version("[1.0.0](link)"), "1.0.0");
    assert_eq!(version("[1.0.0]()"), "1.0.0");
    assert_eq!(version("[1.0.0][link]"), "1.0.0");
    assert_eq!(version("[1.0.0][]"), "1.0.0");

    assert_eq!(version("v[1.0.0]"), "1.0.0");
    assert_eq!(version("v[1.0.0](link)"), "1.0.0");
    assert_eq!(version("v[1.0.0]()"), "1.0.0");
    assert_eq!(version("v[1.0.0][link]"), "1.0.0");
    assert_eq!(version("v[1.0.0][]"), "1.0.0");

    assert_eq!(version("[v1.0.0]"), "1.0.0");
    assert_eq!(version("[v1.0.0](link)"), "1.0.0");
    assert_eq!(version("[v1.0.0]()"), "1.0.0");
    assert_eq!(version("[v1.0.0][link]"), "1.0.0");
    assert_eq!(version("[v1.0.0][]"), "1.0.0");

    assert_eq!(version("[1.0.0] 2022"), "1.0.0");
    assert_eq!(version("[1.0.0](link) 2022"), "1.0.0");
    assert_eq!(version("[1.0.0]() 2022"), "1.0.0");
    assert_eq!(version("[1.0.0][link] 2022"), "1.0.0");
    assert_eq!(version("[1.0.0][] 2022"), "1.0.0");

    assert_eq!(version("[1.0.0 2022]"), "1.0.0");
    assert_eq!(version("[1.0.0 2022](link)"), "1.0.0");
    assert_eq!(version("[1.0.0 2022]()"), "1.0.0");
    assert_eq!(version("[1.0.0 2022][link]"), "1.0.0");
    assert_eq!(version("[1.0.0 2022][]"), "1.0.0");

    // unclosed '['
    assert_eq!(version("[1.0.0"), "1.0.0");
    assert_eq!(version("v[1.0.0"), "1.0.0");
    assert_eq!(version("[v1.0.0"), "1.0.0");

    // The followings are valid links, but it is invalid or odd for our use case (version format)
    // and will be ignored or partially unlinked for now.
    assert_eq!(version("[1.0.0]a(link)"), "1.0.0]a(link)"); // 1.0.0a(link) in full "unlink"
    assert_eq!(version("[1.0.0][]]"), "1.0.0][]]"); // 1.0.0] in full "unlink"
    assert_eq!(version("[1.0.0](link)a"), "1.0.0](link)a"); // 1.0.0a in full "unlink"
}

// See also "Note" section in `unlink` function.
#[test]
fn test_unlink() {
    assert_eq!(unlink("[1.0.0]"), "1.0.0");
    assert_eq!(unlink("[1.0.0](link)"), "1.0.0");
    assert_eq!(unlink("[1.0.0]()"), "1.0.0");
    assert_eq!(unlink("[1.0.0][link]"), "1.0.0");
    assert_eq!(unlink("[1.0.0][]"), "1.0.0");

    // Link without trailing ']': e.g., [1.0.0 2022-01-01]
    assert_eq!(unlink("[1.0.0"), "1.0.0");
    assert_eq!(unlink("[1.0.0(a)"), "1.0.0(a)");

    // Link without leading '[': e.g., [Version 1.0.0]
    assert_eq!(unlink("1.0.0]"), "1.0.0");
    assert_eq!(unlink("1.0.0](link)"), "1.0.0");
    assert_eq!(unlink("1.0.0][link]"), "1.0.0");

    // The followings are valid links, but it is invalid or odd for our use case (version format)
    // and will be ignored or partially unlinked for now.
    assert_eq!(unlink("[1.0.0]a(link)"), "1.0.0]a(link)"); // 1.0.0a(link) in full "unlink"
    assert_eq!(unlink("[1.0.0][]]"), "1.0.0][]]"); // 1.0.0] in full "unlink"
    assert_eq!(unlink("[1.0.0](link)a"), "1.0.0](link)a"); // 1.0.0a in full "unlink"
}
