/*
# Run with libFuzzer
cargo fuzz run --release --features libfuzzer parse

# Run with AFL++
cargo afl build --release --features afl
cargo afl fuzz -i seeds/parse -o out target/release/parse
*/

#![cfg_attr(feature = "libfuzzer", no_main)]

use parse_changelog::parse;

#[cfg(any(
    not(any(feature = "libfuzzer", feature = "afl")),
    all(feature = "libfuzzer", feature = "afl"),
))]
compile_error!("exactly one of 'libfuzzer' or 'afl' feature must be enabled");

#[cfg(feature = "libfuzzer")]
libfuzzer_sys::fuzz_target!(|text: &str| {
    run(text);
});

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|bytes: &[u8]| {
        if let Ok(text) = std::str::from_utf8(bytes) {
            run(text);
        }
    });
}

fn run(text: &str) {
    let _result = parse(text);
}
