// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    io,
    path::{Path, PathBuf},
    str,
    sync::LazyLock,
};

use fs_err as fs;
use proc_macro2::TokenStream;

// Inspired by https://stackoverflow.com/a/63904992.
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name[..name.len() - 3].rsplit_once(':').unwrap().1
    }};
}

pub(crate) fn workspace_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.pop(); // codegen
    dir.pop(); // tools
    dir
}

#[track_caller]
pub(crate) fn header(function_name: &str) -> String {
    // rust-analyzer does not respect outer attribute (#[rustfmt::skip]) on
    // a module without a body and unstable ignore option in .rustfmt.toml.
    // https://github.com/rust-lang/rust-analyzer/issues/10826
    // So use inner attribute under cfg(rustfmt).
    format!(
        "// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by {bin_name}
// ({function_name} function at {file}).
// It is not intended for manual editing.\n
#![cfg_attr(rustfmt, rustfmt::skip)]
",
        bin_name = env!("CARGO_BIN_NAME"),
        file = std::panic::Location::caller().file()
    )
}

#[track_caller]
pub(crate) fn write(
    function_name: &str,
    path: impl AsRef<Path>,
    contents: TokenStream,
) -> io::Result<()> {
    write_raw(function_name, path.as_ref(), format_tokens(contents))
}

#[track_caller]
fn format_tokens(contents: TokenStream) -> Vec<u8> {
    let mut out = prettyplease::unparse(
        &syn::parse2(contents.clone()).unwrap_or_else(|e| panic!("{e} in:\n---\n{contents}\n---")),
    )
    .into_bytes();
    format_macros(&mut out);
    out
}

// Roughly format the code inside macro calls.
fn format_macros(bytes: &mut Vec<u8>) {
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i..].starts_with(b"!(") {
            i += 2;
            let mut count = 0;
            while let Some(b) = bytes.get(i) {
                match b {
                    b'(' => count += 1,
                    b')' => {
                        if count == 0 {
                            break;
                        }
                        count -= 1;
                    }
                    _ => {
                        fn replace(
                            bytes: &mut Vec<u8>,
                            i: usize,
                            needle: &[u8],
                            with: &[u8],
                        ) -> usize {
                            if bytes[i..].starts_with(needle) {
                                bytes.splice(i..i + needle.len(), with.iter().copied());
                                i + with.len() - 1
                            } else {
                                i
                            }
                        }
                        i = replace(bytes, i, b"crate ::", b"crate::");
                        i = replace(bytes, i, b" < ", b"<");
                        i = replace(bytes, i, b" >", b">");
                    }
                }
                i += 1;
            }
        } else {
            i += 1;
        }
    }
}
#[test]
fn test_format_macros() {
    #[track_caller]
    fn t(from: &[u8], expected: &[u8]) {
        let b = &mut from.to_owned();
        format_macros(b);
        assert_eq!(b, expected);
    }
    t(b"m!(crate ::a::b)", b"m!(crate::a::b)");
    t(b"(crate ::a::b)", b"(crate ::a::b)");
    t(b"m!(crate ::a::b < () >)", b"m!(crate::a::b<()>)");
    t(b"m!(crate ::a::b <  >)", b"m!(crate::a::b<>)");
    t(b"if < 0 ", b"if < 0 ");
    t(b"if > 0 ", b"if > 0 ");
}

#[track_caller]
pub(crate) fn write_raw(
    function_name: &str,
    path: &Path,
    contents: impl AsRef<[u8]>,
) -> io::Result<()> {
    static LINGUIST_GENERATED: LazyLock<Vec<globset::GlobMatcher>> = LazyLock::new(|| {
        let gitattributes = fs::read_to_string(workspace_root().join(".gitattributes")).unwrap();
        let mut linguist_generated = vec![];
        for line in gitattributes.lines() {
            if line.contains("linguist-generated") {
                linguist_generated.push(
                    globset::Glob::new(line.split_once(' ').unwrap().0).unwrap().compile_matcher(),
                );
            }
        }
        linguist_generated
    });
    let p = path.strip_prefix(workspace_root()).unwrap();
    if !LINGUIST_GENERATED.iter().any(|m| m.is_match(p)) {
        eprintln!("warning: you may want to mark {} linguist-generated", p.display());
    }

    let mut out = header(function_name).into_bytes();
    out.extend_from_slice(contents.as_ref());
    if path.is_file() && fs::read(path)? == out {
        return Ok(());
    }
    fs::write(path, out)?;
    eprintln!("updated {}", p.display());
    Ok(())
}
