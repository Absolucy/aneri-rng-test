pub mod global;
pub mod instance;
mod key;
pub mod shared;

use byondapi::value::ByondValue;
use eyre::Result;
pub use key::ByondSlotKey;

#[byondapi::bind]
pub fn cleanup() -> Result<ByondValue> {
	instance::free_instances();
	global::reseed_global_rng();
	Ok(ByondValue::null())
}

#[test]
fn generate_binds() {
	byondapi::generate_bindings(env!("CARGO_CRATE_NAME"));
}
