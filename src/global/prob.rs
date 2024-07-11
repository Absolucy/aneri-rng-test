// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use byondapi::value::ByondValue;
use eyre::Result;

#[byondapi::bind]
pub fn prob(probability: ByondValue, secure: ByondValue) -> Result<ByondValue> {
	let probability = probability.get_number()? as f64;
	let secure = secure.is_true();
	Ok(shared::prob(&mut global(secure), probability).into())
}

#[byondapi::bind]
pub fn prob_ratio(
	numerator: ByondValue,
	denominator: ByondValue,
	secure: ByondValue,
) -> Result<ByondValue> {
	let numerator = numerator.get_number()? as u32;
	let denominator = denominator.get_number()? as u32;
	let secure = secure.is_true();
	Ok(shared::prob_ratio(&mut global(secure), numerator, denominator).into())
}
