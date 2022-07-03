#![no_main]
extern crate mugle_core;
#[macro_use]
extern crate libfuzzer_sys;

use mugle_core::core::UntrustedCompactBlock;
use mugle_core::ser;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<UntrustedCompactBlock, ser::Error> =
		ser::deserialize(&mut d, ser::ProtocolVersion(1));
});
