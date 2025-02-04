// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by parse-changelog-internal-codegen
// (gen_track_size function at tools/codegen/src/main.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(dead_code, clippy::std_instead_of_alloc, clippy::std_instead_of_core)]
use std::{fmt::Write as _, path::Path, string::String};
fn write_size<T>(out: &mut String) {
    let _ = writeln!(
        out, "{}: {}", std::any::type_name::<T> (), std::mem::size_of::<T> ()
    );
}
/// Test the size of public types. This is not intended to keep a specific size and
/// is intended to be used only as a help in optimization.
///
/// Ignore non-64-bit targets due to usize/ptr size, and ignore Miri/cargo-careful
/// as we set -Z randomize-layout for them.
#[test]
#[cfg_attr(any(not(target_pointer_width = "64"), miri, careful), ignore)]
fn track_size() {
    let mut out = String::new();
    write_size::<crate::error::Error>(&mut out);
    write_size::<crate::Release<'_>>(&mut out);
    write_size::<crate::Parser>(&mut out);
    test_helper::git::assert_diff(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/gen/tests/track_size.txt"),
        out,
    );
}
