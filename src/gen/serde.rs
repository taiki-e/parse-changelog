// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by parse-changelog-internal-codegen
// (gen_serde_impl function at tools/codegen/src/main.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
use serde::ser::{Serialize, SerializeStruct as _, Serializer};
impl Serialize for crate::Release<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Release", 3)?;
        state.serialize_field("version", &self.version)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("notes", &self.notes)?;
        state.end()
    }
}
