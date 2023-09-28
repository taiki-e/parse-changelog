<!-- markdownlint-disable -->
[8662ab42...HEAD](https://github.com/rust-lang/cargo/compare/8662ab42...HEAD)

### Added
- Added the `doc` field to `cargo metadata`, which indicates if a target is
  documented.
  [#8869](https://github.com/rust-lang/cargo/pull/8869)

### Changed
- `.crate` files uploaded to a registry are now built with reproducible
  settings, so that the same `.crate` file created on different machines
  should be identical.
  [#8864](https://github.com/rust-lang/cargo/pull/8864)

### Fixed

### Nightly only
- Allow `resolver="1"` to specify the original feature resolution behavior.
  [#8857](https://github.com/rust-lang/cargo/pull/8857)
- Added `-Z extra-link-arg` which adds the `cargo:rustc-link-arg-bins`
  and `cargo:rustc-link-arg` build script options.
  [docs](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#extra-link-arg)
  [#8441](https://github.com/rust-lang/cargo/pull/8441)
