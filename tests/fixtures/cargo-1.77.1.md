<!-- markdownlint-disable -->
### Fixed

- Debuginfo is no longer stripped by default for Windows MSVC targets. This caused an unexpected regression in 1.77.0 that broke backtraces.
  [#13654](https://github.com/rust-lang/cargo/pull/13654)