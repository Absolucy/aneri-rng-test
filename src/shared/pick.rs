// SPDX-License-Identifier: MPL-2.0
use byondapi::value::ByondValue;
use eyre::Result;
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};

pub(crate) fn pick<Gen>(rng: &mut Gen, options: ByondValue) -> Result<ByondValue>
where
	Gen: Rng,
{
	if !options.is_list() {
		return Ok(ByondValue::null());
	}
	let length = options.builtin_length()?.get_number()? as usize;
	match length {
		0 => return Ok(ByondValue::null()),
		1 => return Ok(options.read_list_index(1.0)?),
		_ => {}
	}
	let idx = rng.gen_range::<usize, _>(1..=length);
	Ok(options.read_list_index(idx as f32)?)
}

pub(crate) fn pick_weighted<Gen>(rng: &mut Gen, options: ByondValue) -> Result<ByondValue>
where
	Gen: Rng,
{
	if !options.is_list() {
		return Ok(ByondValue::null());
	}
	let length = options.builtin_length()?.get_number()? as usize;
	match length {
		0 => return Ok(ByondValue::null()),
		1 => return Ok(options.read_list_index(1.0)?),
		_ => {}
	}
	let options = options.get_list()?;
	let options_len = options.len();
	if options_len % 2 != 0 {
		return Err(eyre::eyre!("not an assoc list"));
	}
	let mut iter = options.into_iter();
	let mut options = Vec::with_capacity(options_len / 2);
	let mut weights = Vec::with_capacity(options_len / 2);
	while let (Some(key), Some(value)) = (iter.next(), iter.next()) {
		let weight = match value.get_number() {
			Ok(weight) if weight.is_normal() && weight.is_sign_positive() => weight,
			_ => continue,
		};
		options.push(key);
		weights.push(weight);
	}
	match weights.len() {
		0 => return Ok(ByondValue::null()),
		1 => return Ok(options.into_iter().next().unwrap_or_default()),
		_ => {}
	}
	let dist = match WeightedIndex::new(weights) {
		Ok(dist) => dist,
		Err(_) => return Ok(ByondValue::null()),
	};
	let idx = dist.sample(rng);
	Ok(options.into_iter().nth(idx).unwrap_or_default())
}
