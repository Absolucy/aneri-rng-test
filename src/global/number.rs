// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use byondapi::value::ByondValue;
use eyre::Result;
use rand::Rng;

#[byondapi::bind]
pub fn random_byte(secure: ByondValue) -> Result<ByondValue> {
	let secure = secure.is_true();
	Ok(ByondValue::new_num(global(secure).gen::<u8>() as f32))
}

#[byondapi::bind]
pub fn random_float(secure: ByondValue) -> Result<ByondValue> {
	let secure = secure.is_true();
	Ok(ByondValue::new_num(global(secure).gen::<f32>()))
}

#[byondapi::bind]
pub fn random_int_unsigned(secure: ByondValue) -> Result<ByondValue> {
	let secure = secure.is_true();
	Ok(ByondValue::new_num(global(secure).gen::<u32>() as f32))
}

#[byondapi::bind]
pub fn random_int_signed(secure: ByondValue) -> Result<ByondValue> {
	let secure = secure.is_true();
	Ok(ByondValue::new_num(global(secure).gen::<i32>() as f32))
}

#[byondapi::bind]
pub fn random_range_int_unsigned(
	min: ByondValue,
	max: ByondValue,
	secure: ByondValue,
) -> Result<ByondValue> {
	let secure = secure.is_true();
	let min = min.get_number()? as u32;
	let max = max.get_number()? as u32;
	Ok(ByondValue::new_num(
		shared::random_range_int_unsigned(&mut global(secure), min, max) as f32,
	))
}

#[byondapi::bind]
pub fn random_range_int_signed(
	min: ByondValue,
	max: ByondValue,
	secure: ByondValue,
) -> Result<ByondValue> {
	let secure = secure.is_true();
	let min = min.get_number()? as i32;
	let max = max.get_number()? as i32;
	Ok(ByondValue::new_num(
		shared::random_range_int_signed(&mut global(secure), min, max) as f32,
	))
}
