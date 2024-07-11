// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::{shared, ByondSlotKey};
use byondapi::value::ByondValue;
use eyre::{OptionExt, Result};

#[byondapi::bind]
pub fn instanced_random_string_alphanumeric(
	src: ByondValue,
	length: ByondValue,
) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	let length = length.get_number()? as usize;
	INSTANCES
		.lock()
		.get_mut(src)
		.ok_or_eyre("invalid instance")
		.map(|rng| shared::random_string_alphanumeric(rng, length))
		.and_then(|string| ByondValue::new_str(string).map_err(eyre::Error::from))
}

#[byondapi::bind]
pub fn instanced_replace_chars_prob(
	src: ByondValue,
	input: ByondValue,
	replacement: ByondValue,
	prob: ByondValue,
	skip_whitespace: ByondValue,
) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	let input = input.get_string()?;
	let replacement = replacement.get_string()?;
	let prob = prob.get_number()?;
	let skip_whitespace = skip_whitespace.is_true();
	INSTANCES
		.lock()
		.get_mut(src)
		.ok_or_eyre("invalid instance")
		.map(|rng| shared::replace_chars_prob(rng, input, replacement, prob, skip_whitespace))
		.and_then(|string| ByondValue::new_str(string).map_err(eyre::Error::from))
}
