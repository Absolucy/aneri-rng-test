// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::{shared, ByondSlotKey};
use byondapi::value::ByondValue;
use eyre::{OptionExt, Result};

#[byondapi::bind]
pub fn instanced_prob(src: ByondValue, probability: ByondValue) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	let probability = probability.get_number()? as f64;
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::prob(rng, probability))
		.map(ByondValue::from)
		.ok_or_eyre("invalid instance")
}

#[byondapi::bind]
pub fn instanced_prob_ratio(
	src: ByondValue,
	numerator: ByondValue,
	denominator: ByondValue,
) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	let numerator = numerator.get_number()? as u32;
	let denominator = denominator.get_number()? as u32;
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::prob_ratio(rng, numerator, denominator))
		.map(ByondValue::from)
		.ok_or_eyre("invalid instance")
}
