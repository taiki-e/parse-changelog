#![no_main]

use std::str;

use libfuzzer_sys::fuzz_target;
use parse_changelog::parse;

fuzz_target!(|string: &str| {
    let _result = parse(string);
});
