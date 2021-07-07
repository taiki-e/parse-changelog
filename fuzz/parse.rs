#![no_main]

use std::str;

use libfuzzer_sys::fuzz_target;
use parse_changelog::parse;

fuzz_target!(|bytes: &[u8]| {
    if let Ok(string) = str::from_utf8(bytes) {
        let _result = parse(string);
    }
});
