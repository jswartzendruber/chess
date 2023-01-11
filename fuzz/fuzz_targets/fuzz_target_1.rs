#![no_main]

use libfuzzer_sys::fuzz_target;
extern crate chiss;

fuzz_target!(|data: &str| {
    chiss::parse_move(data);
});
