// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use byondapi::value::ByondValue;
use eyre::Result;

#[byondapi::bind]
pub fn random_string_alphanumeric(length: ByondValue, secure: ByondValue) -> Result<ByondValue> {
	let length = length.get_number()? as usize;
	let secure = secure.is_true();
	Ok(ByondValue::new_str(shared::random_string_alphanumeric(
		&mut global(secure),
		length,
	))?)
}

#[byondapi::bind]
pub fn replace_chars_prob(
	input: ByondValue,
	replacement: ByondValue,
	prob: ByondValue,
	skip_whitespace: ByondValue,
	secure: ByondValue,
) -> Result<ByondValue> {
	let input = input.get_string()?;
	let replacement = replacement.get_string()?;
	let prob = prob.get_number()?;
	let skip_whitespace = skip_whitespace.is_true();
	let secure = secure.is_true();
	Ok(ByondValue::new_str(shared::replace_chars_prob(
		&mut global(secure),
		input,
		replacement,
		prob,
		skip_whitespace,
	))?)
}
