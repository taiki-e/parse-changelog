// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Run with libFuzzer:

```sh
cargo fuzz run --release --features libfuzzer parse
```

Run with AFL++:

```sh
cd fuzz
cargo afl build --release --features afl
cargo afl fuzz -i seeds/parse -o out/parse target/release/parse
```

Run with Honggfuzz:

```sh
cd fuzz
HFUZZ_RUN_ARGS="${HFUZZ_RUN_ARGS:-} --exit_upon_crash" \
    HFUZZ_BUILD_ARGS="${HFUZZ_BUILD_ARGS:-} --features honggfuzz" \
    RUSTFLAGS="${RUSTFLAGS:-} -Z sanitizer=address" \
    cargo hfuzz run parse
```

Run with AFL++ + cargo-llvm-cov:

```sh
cd fuzz
source <(cargo llvm-cov show-env --export-prefix)
cargo llvm-cov clean --workspace
cargo afl build --release --features afl
AFL_FUZZER_LOOPCOUNT=20 \
    cargo afl fuzz -c - -i seeds/parse -o out/parse target/release/parse
cargo llvm-cov report --release --open
```
*/

#![cfg_attr(feature = "libfuzzer", no_main)]

use parse_changelog::parse;

#[cfg(any(
    not(any(feature = "libfuzzer", feature = "afl", feature = "honggfuzz")),
    all(feature = "libfuzzer", feature = "afl"),
    all(feature = "libfuzzer", feature = "honggfuzz"),
    all(feature = "afl", feature = "honggfuzz"),
))]
compile_error!("exactly one of 'libfuzzer' or 'afl' or 'honggfuzz' feature must be enabled");

#[cfg(feature = "libfuzzer")]
libfuzzer_sys::fuzz_target!(|bytes| run(bytes));
#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|bytes| run(bytes));
}
#[cfg(feature = "honggfuzz")]
fn main() {
    loop {
        honggfuzz::fuzz!(|bytes| { run(bytes) });
    }
}

fn run(bytes: &[u8]) {
    let Ok(text) = std::str::from_utf8(bytes) else { return };
    let Ok(changelog) = parse(text) else { return };
    for release in changelog.values() {
        let _ = release.title_no_link();
    }
}
