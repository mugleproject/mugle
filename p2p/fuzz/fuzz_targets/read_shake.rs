#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate mugle_core;
extern crate mugle_p2p;

use mugle_core::ser;
use mugle_p2p::msg::Shake;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<Shake, ser::Error> = ser::deserialize(&mut d);
});
