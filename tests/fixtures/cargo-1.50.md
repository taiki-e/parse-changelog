<!-- markdownlint-disable -->
[8662ab42...rust-1.50.0](https://github.com/rust-lang/cargo/compare/8662ab42...rust-1.50.0)

### Added
- Added the `doc` field to `cargo metadata`, which indicates if a target is
  documented.
  [#8869](https://github.com/rust-lang/cargo/pull/8869)
- Added `RUSTC_WORKSPACE_WRAPPER`, an alternate RUSTC wrapper that only runs
  for the local workspace packages, and caches its artifacts independently of
  non-wrapped builds.
  [#8976](https://github.com/rust-lang/cargo/pull/8976)
- Added `--workspace` to `cargo update` to update only the workspace members,
  and not their dependencies. This is particularly useful if you update the
  version in `Cargo.toml` and want to update `Cargo.lock` without running any
  other commands.
  [#8725](https://github.com/rust-lang/cargo/pull/8725)

### Changed
- `.crate` files uploaded to a registry are now built with reproducible
  settings, so that the same `.crate` file created on different machines
  should be identical.
  [#8864](https://github.com/rust-lang/cargo/pull/8864)
- Git dependencies that specify more than one of `branch`, `tag`, or `rev` are
  now rejected.
  [#8984](https://github.com/rust-lang/cargo/pull/8984)
- The `rerun-if-changed` build script directive can now point to a directory,
  in which case Cargo will check if any file in that directory changes.
  [#8973](https://github.com/rust-lang/cargo/pull/8973)
- If Cargo cannot determine the username or email address, `cargo new` will no
  longer fail, and instead create an empty authors list.
  [#8912](https://github.com/rust-lang/cargo/pull/8912)
- The progress bar width has been reduced to provide more room to display the
  crates currently being built.
  [#8892](https://github.com/rust-lang/cargo/pull/8892)
- `cargo new` will now support `includeIf` directives in `.gitconfig` to match
  the correct directory when determining the username and email address.
  [#8886](https://github.com/rust-lang/cargo/pull/8886)

### Fixed
- Fixed `cargo metadata` and `cargo tree` to only download packages for the
  requested target.
  [#8987](https://github.com/rust-lang/cargo/pull/8987)
- Updated libgit2, which brings in many fixes, particularly fixing a zlib
  error that occasionally appeared on 32-bit systems.
  [#8998](https://github.com/rust-lang/cargo/pull/8998)
- Fixed stack overflow with a circular dev-dependency that uses the `links`
  field.
  [#8969](https://github.com/rust-lang/cargo/pull/8969)
- Fixed `cargo publish` failing on some filesystems, particularly 9p on WSL2.
  [#8950](https://github.com/rust-lang/cargo/pull/8950)

### Nightly only
- Allow `resolver="1"` to specify the original feature resolution behavior.
  [#8857](https://github.com/rust-lang/cargo/pull/8857)
- Added `-Z extra-link-arg` which adds the `cargo:rustc-link-arg-bins`
  and `cargo:rustc-link-arg` build script options.
  [docs](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#extra-link-arg)
  [#8441](https://github.com/rust-lang/cargo/pull/8441)
- Implemented external credential process support, and added `cargo logout`.
  ([RFC 2730](https://github.com/rust-lang/rfcs/blob/master/text/2730-cargo-token-from-process.md))
  ([docs](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#credential-process))
  [#8934](https://github.com/rust-lang/cargo/pull/8934)
- Fix panic with `-Zbuild-std` and no roots.
  [#8942](https://github.com/rust-lang/cargo/pull/8942)
- Set docs.rs as the default extern-map for crates.io
  [#8877](https://github.com/rust-lang/cargo/pull/8877)