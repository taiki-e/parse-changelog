use std::str;

use afl::fuzz;
use parse_changelog::parse;

fn main() {
    fuzz!(|bytes: &[u8]| {
        if let Ok(string) = str::from_utf8(bytes) {
            let _result = parse(string);
        }
    });
}
