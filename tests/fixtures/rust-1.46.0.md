Language
--------
- [`if`, `match`, and `loop` expressions can now be used in const functions.][72437]
- [Additionally you are now also able to coerce and cast to slices (`&[T]`) in
  const functions.][73862]
- [The `#[track_caller]` attribute can now be added to functions to use the
  function's caller's location information for panic messages.][72445]
- [Recursively indexing into tuples no longer needs parentheses.][71322] E.g.
  `x.0.0` over `(x.0).0`.
- [`mem::transmute` can now be used in statics and constants.][72920] **Note**
  You currently can't use `mem::transmute` in constant functions.

Compiler
--------
- [You can now use the `cdylib` target on Apple iOS and tvOS platforms.][73516]
- [Enabled static "Position Independent Executables" by default
  for `x86_64-unknown-linux-musl`.][70740]

Libraries
---------
- [`mem::forget` is now a `const fn`.][73887]
- [`String` now implements `From<char>`.][73466]
- [The `leading_ones`, and `trailing_ones` methods have been stabilised for all
  integer types.][73032]
- [`vec::IntoIter<T>` now implements `AsRef<[T]>`.][72583]
- [All non-zero integer types (`NonZeroU8`) now implement `TryFrom` for their
  zero-able equivalent (e.g. `TryFrom<u8>`).][72717]
- [`&[T]` and `&mut [T]` now implement `PartialEq<Vec<T>>`.][71660]
- [`(String, u16)` now implements `ToSocketAddrs`.][73007]
- [`vec::Drain<'_, T>` now implements `AsRef<[T]>`.][72584]

Stabilized APIs
---------------
- [`Option::zip`]
- [`vec::Drain::as_slice`]

Cargo
-----
Added a number of new environment variables that are now available when
compiling your crate.

- [`CARGO_BIN_NAME` and `CARGO_CRATE_NAME`][cargo/8270] Providing the name of
  the specific binary being compiled and the name of the crate.
- [`CARGO_PKG_LICENSE`][cargo/8325] The license from the manifest of the package.
- [`CARGO_PKG_LICENSE_FILE`][cargo/8387] The path to the license file.

Compatibility Notes
-------------------
- [The target configuration option `abi_blacklist` has been renamed
  to `unsupported_abis`.][74150] The old name will still continue to work.
- [Rustc will now warn if you cast a C-like enum that implements `Drop`.][72331]
  This was previously accepted but will become a hard error in a future release.
- [Rustc will fail to compile if you have a struct with
  `#[repr(i128)]` or `#[repr(u128)]`.][74109] This representation is currently only
  allowed on `enum`s.
- [Tokens passed to `macro_rules!` are now always captured.][73293] This helps
  ensure that spans have the correct information, and may cause breakage if you
  were relying on receiving spans with dummy information.
- [The InnoSetup installer for Windows is no longer available.][72569] This was
  a legacy installer that was replaced by a MSI installer a few years ago but
  was still being built.
- [`{f32, f64}::asinh` now returns the correct values for negative numbers.][72486]
- [Rustc will no longer accept overlapping trait implementations that only
  differ in how the lifetime was bound.][72493]
- [Rustc now correctly relates the lifetime of an existential associated
  type.][71896] This fixes some edge cases where `rustc` would erroneously allow
  you to pass a shorter lifetime than expected.
- [Rustc now dynamically links to `libz` (also called `zlib`) on Linux.][74420]
  The library will need to be installed for `rustc` to work, even though we
  expect it to be already available on most systems.
- [Tests annotated with `#[should_panic]` are broken on ARMv7 while running
  under QEMU.][74820]
- [Pretty printing of some tokens in procedural macros changed.][75453] The
  exact output returned by rustc's pretty printing is an unstable
  implementation detail: we recommend any macro relying on it to switch to a
  more robust parsing system.

[75453]: https://github.com/rust-lang/rust/issues/75453/
[74820]: https://github.com/rust-lang/rust/issues/74820/
[74420]: https://github.com/rust-lang/rust/issues/74420/
[74109]: https://github.com/rust-lang/rust/pull/74109/
[74150]: https://github.com/rust-lang/rust/pull/74150/
[73862]: https://github.com/rust-lang/rust/pull/73862/
[73887]: https://github.com/rust-lang/rust/pull/73887/
[73466]: https://github.com/rust-lang/rust/pull/73466/
[73516]: https://github.com/rust-lang/rust/pull/73516/
[73293]: https://github.com/rust-lang/rust/pull/73293/
[73007]: https://github.com/rust-lang/rust/pull/73007/
[73032]: https://github.com/rust-lang/rust/pull/73032/
[72920]: https://github.com/rust-lang/rust/pull/72920/
[72569]: https://github.com/rust-lang/rust/pull/72569/
[72583]: https://github.com/rust-lang/rust/pull/72583/
[72584]: https://github.com/rust-lang/rust/pull/72584/
[72717]: https://github.com/rust-lang/rust/pull/72717/
[72437]: https://github.com/rust-lang/rust/pull/72437/
[72445]: https://github.com/rust-lang/rust/pull/72445/
[72486]: https://github.com/rust-lang/rust/pull/72486/
[72493]: https://github.com/rust-lang/rust/pull/72493/
[72331]: https://github.com/rust-lang/rust/pull/72331/
[71896]: https://github.com/rust-lang/rust/pull/71896/
[71660]: https://github.com/rust-lang/rust/pull/71660/
[71322]: https://github.com/rust-lang/rust/pull/71322/
[70740]: https://github.com/rust-lang/rust/pull/70740/
[cargo/8270]: https://github.com/rust-lang/cargo/pull/8270/
[cargo/8325]: https://github.com/rust-lang/cargo/pull/8325/
[cargo/8387]: https://github.com/rust-lang/cargo/pull/8387/
[`Option::zip`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.zip
[`vec::Drain::as_slice`]: https://doc.rust-lang.org/stable/std/vec/struct.Drain.html#method.as_slice
