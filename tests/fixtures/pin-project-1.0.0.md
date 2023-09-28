<!-- markdownlint-disable -->
- [Remove deprecated `#[project]`, `#[project_ref]`, and `#[project_replace]` attributes.](https://github.com/taiki-e/pin-project/pull/265)

  Name the projected type by passing an argument with the same name as the method to the `#[pin_project]` attribute instead:

  ```diff
  - #[pin_project]
  + #[pin_project(project = EnumProj)]
    enum Enum<T> {
        Variant(#[pin] T),
    }

  - #[project]
    fn func<T>(x: Pin<&mut Enum<T>>) {
  -     #[project]
        match x.project() {
  -         Enum::Variant(_) => { /* ... */ }
  +         EnumProj::Variant(_) => { /* ... */ }
        }
    }
  ```

- [Remove deprecated `Replace` argument from `#[pin_project]` attribute.](https://github.com/taiki-e/pin-project/pull/266) Use `project_replace` argument instead.

- [Optimize code generation when used on enums.](https://github.com/taiki-e/pin-project/pull/270)

- [Raise the minimum supported Rust version of this crate from Rust 1.34 to Rust 1.37.](https://github.com/taiki-e/pin-project/pull/292)

- Suppress `explicit_outlives_requirements`, `box_pointers`, `clippy::large_enum_variant`, `clippy::pattern_type_mismatch`, `clippy::implicit_return`, and `clippy::redundant_pub_crate` lints in generated code. ([#276](https://github.com/taiki-e/pin-project/pull/276), [#277](https://github.com/taiki-e/pin-project/pull/277), [#284](https://github.com/taiki-e/pin-project/pull/284))

- Diagnostic improvements.

Changes since the 1.0.0-alpha.1 release:

- [Fix drop order of pinned fields in `project_replace`.](https://github.com/taiki-e/pin-project/pull/287)

- Update minimal version of `syn` to 1.0.44
