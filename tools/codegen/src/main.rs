// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::needless_pass_by_value, clippy::wildcard_imports)]

use std::{collections::HashSet, path::Path};

use fs_err as fs;
use proc_macro2::Literal;
use quote::{format_ident, quote};
use test_helper::{bin_name, codegen::file, function_name};

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR").strip_suffix("tools/codegen").unwrap())
}

fn main() {
    gen_serde_impl();
    gen_assert_impl();
    gen_track_size();
}

fn gen_serde_impl() {
    const FILES: &[&str] = &["src/lib.rs"];
    const EXCLUDE: &[&str] = &["Parser", "ParseIter"];

    let workspace_root = workspace_root();

    let mut tokens = quote! {
        use serde_core::ser::{Serialize, SerializeStruct as _, Serializer};
    };

    let mut visited_types = HashSet::new();
    for &f in FILES {
        let s = fs::read_to_string(workspace_root.join(f)).unwrap();
        let ast = syn::parse_file(&s).unwrap();

        let module = if f.ends_with("lib.rs") {
            vec![]
        } else {
            let name = format_ident!("{}", Path::new(f).file_stem().unwrap().to_string_lossy());
            vec![name.into()]
        };

        test_helper::codegen::visit_items(module, ast, |item, module| match item {
            syn::Item::Struct(syn::ItemStruct { vis, ident, generics, fields, .. })
                if matches!(vis, syn::Visibility::Public(..))
                    && matches!(fields, syn::Fields::Named(..)) =>
            {
                let path_string = quote! { #(#module::)* #ident }.to_string().replace(' ', "");
                visited_types.insert(path_string.clone());
                if !EXCLUDE.contains(&path_string.as_str()) {
                    assert_eq!(
                        generics.type_params().count(),
                        0,
                        "gen_serde_impl doesn't support generics yet; consider excluding `{path_string}`"
                    );
                    assert_eq!(
                        generics.const_params().count(),
                        0,
                        "gen_serde_impl doesn't support const generics yet; consider excluding `{path_string}`"
                    );
                    let num_fields = Literal::usize_unsuffixed(fields.len());
                    let fields = fields.iter().map(|syn::Field { ident, .. }| {
                        let name = ident.as_ref().unwrap().to_string();
                        quote! { state.serialize_field(#name, &self.#ident)?; }
                    });
                    let name = ident.to_string();
                    let lt = generics.lifetimes().map(|_| quote! { '_ });
                    tokens.extend(quote! {
                        impl Serialize for crate:: #(#module::)* #ident <#(#lt),*> {
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                            where
                                S: Serializer,
                            {
                                let mut state = serializer.serialize_struct(#name, #num_fields)?;
                                #(#fields)*
                                state.end()
                            }
                        }
                    });
                }
            }
            _ => {}
        });
    }

    for &t in EXCLUDE {
        assert!(visited_types.contains(t), "unknown type `{t}` specified in EXCLUDE constant");
    }

    file::write(
        function_name!(),
        bin_name!(),
        workspace_root,
        workspace_root.join("src/gen/serde.rs"),
        tokens,
    );
}

fn gen_assert_impl() {
    let workspace_root = workspace_root();
    let (path, out) = test_helper::codegen::gen_assert_impl(
        workspace_root,
        test_helper::codegen::AssertImplConfig {
            exclude: &[],
            not_send: &[],
            not_sync: &[],
            not_unpin: &[],
            not_unwind_safe: &[],
            not_ref_unwind_safe: &[],
        },
    );
    file::write(function_name!(), bin_name!(), workspace_root, path, out);
}

fn gen_track_size() {
    let workspace_root = workspace_root();
    let (path, out) = test_helper::codegen::gen_track_size(
        workspace_root,
        test_helper::codegen::TrackSizeConfig {
            exclude: &["ParseIter"], // size different between AArch64 and x86_64
        },
    );
    file::write(function_name!(), bin_name!(), workspace_root, path, out);
}
