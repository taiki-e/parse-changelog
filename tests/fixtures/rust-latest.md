<!-- markdownlint-disable -->
<a id="1.80-Language"></a>

Language
--------
- [Document maximum allocation size](https://github.com/rust-lang/rust/pull/116675/)
- [Allow zero-byte offsets and ZST read/writes on arbitrary pointers](https://github.com/rust-lang/rust/pull/117329/)
- [Support C23's variadics without a named parameter](https://github.com/rust-lang/rust/pull/124048/)
- [Stabilize `exclusive_range_pattern` feature](https://github.com/rust-lang/rust/pull/124459/)
- [Guarantee layout and ABI of `Result` in some scenarios](https://github.com/rust-lang/rust/pull/124870)

<a id="1.80-Compiler"></a>

Compiler
--------
- [Update cc crate to v1.0.97 allowing additional spectre mitigations on MSVC targets](https://github.com/rust-lang/rust/pull/124892/)
- [Allow field reordering on types marked `repr(packed(1))`](https://github.com/rust-lang/rust/pull/125360/)
- [Add a lint against never type fallback affecting unsafe code](https://github.com/rust-lang/rust/pull/123939/)
- [Disallow cast with trailing braced macro in let-else](https://github.com/rust-lang/rust/pull/125049/)
- [Expand `for_loops_over_fallibles` lint to lint on fallibles behind references.](https://github.com/rust-lang/rust/pull/125156/)
- [self-contained linker: retry linking without `-fuse-ld=lld` on CCs that don't support it](https://github.com/rust-lang/rust/pull/125417/)
- [Do not parse CVarArgs (`...`) as a type in trait bounds](https://github.com/rust-lang/rust/pull/125863/)
- Improvements to LLDB formatting [#124458](https://github.com/rust-lang/rust/pull/124458) [#124500](https://github.com/rust-lang/rust/pull/124500)
- [For the wasm32-wasip2 target default to PIC and do not use `-fuse-ld=lld`](https://github.com/rust-lang/rust/pull/124858/)
- [Add x86_64-unknown-linux-none as a tier 3 target](https://github.com/rust-lang/rust/pull/125023/)
- [Lint on `foo.into_iter()` resolving to `&Box<[T]>: IntoIterator`](https://github.com/rust-lang/rust/pull/124097/)

<a id="1.80-Libraries"></a>

Libraries
---------
- [Add `size_of` and `size_of_val` and `align_of` and `align_of_val` to the prelude](https://github.com/rust-lang/rust/pull/123168/)
- [Abort a process when FD ownership is violated](https://github.com/rust-lang/rust/pull/124210/)
- [io::Write::write_fmt: panic if the formatter fails when the stream does not fail](https://github.com/rust-lang/rust/pull/125012/)
- [Panic if `PathBuf::set_extension` would add a path separator](https://github.com/rust-lang/rust/pull/125070/)
- [Add assert_unsafe_precondition to unchecked_{add,sub,neg,mul,shl,shr} methods](https://github.com/rust-lang/rust/pull/121571/)
- [Update `c_char` on AIX to use the correct type](https://github.com/rust-lang/rust/pull/122986/)
- [`offset_of!` no longer returns a temporary](https://github.com/rust-lang/rust/pull/124484/)
- [Handle sigma in `str.to_lowercase` correctly](https://github.com/rust-lang/rust/pull/124773/)
- [Raise `DEFAULT_MIN_STACK_SIZE` to at least 64KiB](https://github.com/rust-lang/rust/pull/126059/)

<a id="1.80-Stabilized-APIs"></a>

Stabilized APIs
---------------
- [`impl Default for Rc<CStr>`](https://doc.rust-lang.org/beta/alloc/rc/struct.Rc.html#impl-Default-for-Rc%3CCStr%3E)
- [`impl Default for Rc<str>`](https://doc.rust-lang.org/beta/alloc/rc/struct.Rc.html#impl-Default-for-Rc%3Cstr%3E)
- [`impl Default for Rc<[T]>`](https://doc.rust-lang.org/beta/alloc/rc/struct.Rc.html#impl-Default-for-Rc%3C%5BT%5D%3E)
- [`impl Default for Arc<str>`](https://doc.rust-lang.org/beta/alloc/sync/struct.Arc.html#impl-Default-for-Arc%3Cstr%3E)
- [`impl Default for Arc<CStr>`](https://doc.rust-lang.org/beta/alloc/sync/struct.Arc.html#impl-Default-for-Arc%3CCStr%3E)
- [`impl Default for Arc<[T]>`](https://doc.rust-lang.org/beta/alloc/sync/struct.Arc.html#impl-Default-for-Arc%3C%5BT%5D%3E)
- [`impl IntoIterator for Box<[T]>`](https://doc.rust-lang.org/beta/alloc/boxed/struct.Box.html#impl-IntoIterator-for-Box%3C%5BI%5D,+A%3E)
- [`impl FromIterator<String> for Box<str>`](https://doc.rust-lang.org/beta/alloc/boxed/struct.Box.html#impl-FromIterator%3CString%3E-for-Box%3Cstr%3E)
- [`impl FromIterator<char> for Box<str>`](https://doc.rust-lang.org/beta/alloc/boxed/struct.Box.html#impl-FromIterator%3Cchar%3E-for-Box%3Cstr%3E)
- [`LazyCell`](https://doc.rust-lang.org/beta/core/cell/struct.LazyCell.html)
- [`LazyLock`](https://doc.rust-lang.org/beta/std/sync/struct.LazyLock.html)
- [`Duration::div_duration_f32`](https://doc.rust-lang.org/beta/std/time/struct.Duration.html#method.div_duration_f32)
- [`Duration::div_duration_f64`](https://doc.rust-lang.org/beta/std/time/struct.Duration.html#method.div_duration_f64)
- [`Option::take_if`](https://doc.rust-lang.org/beta/std/option/enum.Option.html#method.take_if)
- [`Seek::seek_relative`](https://doc.rust-lang.org/beta/std/io/trait.Seek.html#method.seek_relative)
- [`BinaryHeap::as_slice`](https://doc.rust-lang.org/beta/std/collections/struct.BinaryHeap.html#method.as_slice)
- [`NonNull::offset`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.offset)
- [`NonNull::byte_offset`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.byte_offset)
- [`NonNull::add`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.add)
- [`NonNull::byte_add`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.byte_add)
- [`NonNull::sub`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.sub)
- [`NonNull::byte_sub`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.byte_sub)
- [`NonNull::offset_from`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.offset_from)
- [`NonNull::byte_offset_from`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.byte_offset_from)
- [`NonNull::read`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.read)
- [`NonNull::read_volatile`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.read_volatile)
- [`NonNull::read_unaligned`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.read_unaligned)
- [`NonNull::write`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.write)
- [`NonNull::write_volatile`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.write_volatile)
- [`NonNull::write_unaligned`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.write_unaligned)
- [`NonNull::write_bytes`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.write_bytes)
- [`NonNull::copy_to`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.copy_to)
- [`NonNull::copy_to_nonoverlapping`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.copy_to_nonoverlapping)
- [`NonNull::copy_from`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.copy_from)
- [`NonNull::copy_from_nonoverlapping`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.copy_from_nonoverlapping)
- [`NonNull::replace`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.replace)
- [`NonNull::swap`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.swap)
- [`NonNull::drop_in_place`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.drop_in_place)
- [`NonNull::align_offset`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.align_offset)
- [`<[T]>::split_at_checked`](https://doc.rust-lang.org/beta/std/primitive.slice.html#method.split_at_checked)
- [`<[T]>::split_at_mut_checked`](https://doc.rust-lang.org/beta/std/primitive.slice.html#method.split_at_mut_checked)
- [`str::split_at_checked`](https://doc.rust-lang.org/beta/std/primitive.str.html#method.split_at_checked)
- [`str::split_at_mut_checked`](https://doc.rust-lang.org/beta/std/primitive.str.html#method.split_at_mut_checked)
- [`str::trim_ascii`](https://doc.rust-lang.org/beta/std/primitive.str.html#method.trim_ascii)
- [`str::trim_ascii_start`](https://doc.rust-lang.org/beta/std/primitive.str.html#method.trim_ascii_start)
- [`str::trim_ascii_end`](https://doc.rust-lang.org/beta/std/primitive.str.html#method.trim_ascii_end)
- [`<[u8]>::trim_ascii`](https://doc.rust-lang.org/beta/core/primitive.slice.html#method.trim_ascii)
- [`<[u8]>::trim_ascii_start`](https://doc.rust-lang.org/beta/core/primitive.slice.html#method.trim_ascii_start)
- [`<[u8]>::trim_ascii_end`](https://doc.rust-lang.org/beta/core/primitive.slice.html#method.trim_ascii_end)
- [`Ipv4Addr::BITS`](https://doc.rust-lang.org/beta/core/net/struct.Ipv4Addr.html#associatedconstant.BITS)
- [`Ipv4Addr::to_bits`](https://doc.rust-lang.org/beta/core/net/struct.Ipv4Addr.html#method.to_bits)
- [`Ipv4Addr::from_bits`](https://doc.rust-lang.org/beta/core/net/struct.Ipv4Addr.html#method.from_bits)
- [`Ipv6Addr::BITS`](https://doc.rust-lang.org/beta/core/net/struct.Ipv6Addr.html#associatedconstant.BITS)
- [`Ipv6Addr::to_bits`](https://doc.rust-lang.org/beta/core/net/struct.Ipv6Addr.html#method.to_bits)
- [`Ipv6Addr::from_bits`](https://doc.rust-lang.org/beta/core/net/struct.Ipv6Addr.html#method.from_bits)
- [`Vec::<[T; N]>::into_flattened`](https://doc.rust-lang.org/beta/alloc/vec/struct.Vec.html#method.into_flattened)
- [`<[[T; N]]>::as_flattened`](https://doc.rust-lang.org/beta/core/primitive.slice.html#method.as_flattened)
- [`<[[T; N]]>::as_flattened_mut`](https://doc.rust-lang.org/beta/core/primitive.slice.html#method.as_flattened_mut)

These APIs are now stable in const contexts:

- [`<[T]>::last_chunk`](https://doc.rust-lang.org/beta/core/primitive.slice.html#method.last_chunk)
- [`BinaryHeap::new`](https://doc.rust-lang.org/beta/std/collections/struct.BinaryHeap.html#method.new)

<a id="1.80-Cargo"></a>

Cargo
-----
- [Stabilize `-Zcheck-cfg` as always enabled](https://github.com/rust-lang/cargo/pull/13571/)
- [Warn, rather than fail publish, if a target is excluded](https://github.com/rust-lang/cargo/pull/13713/)
- [Add special `check-cfg` lint config for the `unexpected_cfgs` lint](https://github.com/rust-lang/cargo/pull/13913/)
- [Stabilize `cargo update --precise <yanked>`](https://github.com/rust-lang/cargo/pull/13974/)
- [Don't change file permissions on `Cargo.toml` when using `cargo add`](https://github.com/rust-lang/cargo/pull/13898/)
- [Support using `cargo fix` on IPv6-only networks](https://github.com/rust-lang/cargo/pull/13907/)

<a id="1.80-Rustdoc"></a>

Rustdoc
-----

- [Allow searching for references](https://github.com/rust-lang/rust/pull/124148/)
- [Stabilize `custom_code_classes_in_docs` feature](https://github.com/rust-lang/rust/pull/124577/)
- [fix: In cross-crate scenarios show enum variants on type aliases of enums](https://github.com/rust-lang/rust/pull/125300/)

<a id="1.80-Compatibility-Notes"></a>

Compatibility Notes
-------------------
- [rustfmt estimates line lengths differently when using non-ascii characters](https://github.com/rust-lang/rustfmt/issues/6203)
- [Type aliases are now handled correctly in orphan check](https://github.com/rust-lang/rust/pull/117164/)
- [Allow instructing rustdoc to read from stdin via `-`](https://github.com/rust-lang/rust/pull/124611/)
- [`std::env::{set_var, remove_var}` can no longer be converted to safe function pointers and no longer implement the `Fn` family of traits](https://github.com/rust-lang/rust/pull/124636)
- [Warn (or error) when `Self` constructor from outer item is referenced in inner nested item](https://github.com/rust-lang/rust/pull/124187/)
- [Turn `indirect_structural_match` and `pointer_structural_match` lints into hard errors](https://github.com/rust-lang/rust/pull/124661/)
- [Make `where_clause_object_safety` lint a regular object safety violation](https://github.com/rust-lang/rust/pull/125380/)
- [Turn `proc_macro_back_compat` lint into a hard error.](https://github.com/rust-lang/rust/pull/125596/)
- [Detect unused structs even when implementing private traits](https://github.com/rust-lang/rust/pull/122382/)
- [`std::sync::ReentrantLockGuard<T>` is no longer `Sync` if `T: !Sync`](https://github.com/rust-lang/rust/pull/125527) which means [`std::io::StdoutLock` and `std::io::StderrLock` are no longer Sync](https://github.com/rust-lang/rust/issues/127340)

<a id="1.80-Internal-Changes"></a>

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- Misc improvements to size of generated html by rustdoc e.g. [#124738](https://github.com/rust-lang/rust/pull/124738/) and [#123734](https://github.com/rust-lang/rust/pull/123734/)
- [MSVC targets no longer depend on libc](https://github.com/rust-lang/rust/pull/124050/)
