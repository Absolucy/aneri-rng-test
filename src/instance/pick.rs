// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::{shared, ByondSlotKey};
use byondapi::value::ByondValue;
use eyre::{OptionExt, Result};

#[byondapi::bind]
pub fn instanced_pick(src: ByondValue, options: ByondValue) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	INSTANCES
		.lock()
		.get_mut(src)
		.ok_or_eyre("invalid instance")
		.and_then(|rng| shared::pick(rng, options))
}

#[byondapi::bind]
pub fn instanced_pick_weighted(src: ByondValue, options: ByondValue) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	INSTANCES
		.lock()
		.get_mut(src)
		.ok_or_eyre("invalid instance")
		.and_then(|rng: &mut super::dispatcher::RngDispatcher| shared::pick_weighted(rng, options))
}
