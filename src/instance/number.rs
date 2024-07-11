// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::{shared, ByondSlotKey};
use byondapi::value::ByondValue;
use eyre::{OptionExt, Result};
use rand::{
	distributions::{Distribution, Standard},
	Rng,
};

fn rand_impl<Output>(src: ByondValue) -> Result<Output>
where
	Standard: Distribution<Output>,
{
	let src = ByondSlotKey::load(&src)?;
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| rng.gen())
		.ok_or_eyre("invalid instance")
}

#[byondapi::bind]
pub fn instanced_random_byte(src: ByondValue) -> Result<ByondValue> {
	rand_impl::<u8>(src).map(|num| ByondValue::new_num(num.into()))
}

#[byondapi::bind]
pub fn instanced_random_float(src: ByondValue) -> Result<ByondValue> {
	rand_impl::<f32>(src).map(ByondValue::new_num)
}

#[byondapi::bind]
pub fn instanced_random_int_unsigned(src: ByondValue) -> Result<ByondValue> {
	rand_impl::<u32>(src).map(|num| ByondValue::new_num(num as f32))
}

#[byondapi::bind]
pub fn instanced_random_int_signed(src: ByondValue) -> Result<ByondValue> {
	rand_impl::<i32>(src).map(|num| ByondValue::new_num(num as f32))
}

#[byondapi::bind]
pub fn instanced_random_range_int_unsigned(
	src: ByondValue,
	min: ByondValue,
	max: ByondValue,
) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	let min = min.get_number()? as u32;
	let max = max.get_number()? as u32;
	INSTANCES
		.lock()
		.get_mut(src)
		.ok_or_eyre("invalid instance")
		.map(|rng| shared::random_range_int_unsigned(rng, min, max))
		.map(|num| ByondValue::new_num(num as f32))
}

#[byondapi::bind]
pub fn instanced_random_range_int_signed(
	src: ByondValue,
	min: ByondValue,
	max: ByondValue,
) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	let min = min.get_number()? as i32;
	let max = max.get_number()? as i32;
	INSTANCES
		.lock()
		.get_mut(src)
		.ok_or_eyre("invalid instance")
		.map(|rng| shared::random_range_int_signed(rng, min, max))
		.map(|num| ByondValue::new_num(num as f32))
}
