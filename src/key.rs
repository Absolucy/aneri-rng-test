// SPDX-License-Identifier: MPL-2.0
use byondapi::value::ByondValue;
use eyre::Result;
use slotmap::{new_key_type, Key, KeyData};

new_key_type! { pub struct ByondSlotKey; }

impl ByondSlotKey {
	const HIGH_VAR: &'static str = "__aneri_key_high";
	const LOW_VAR: &'static str = "__aneri_key_low";

	pub fn save(&self, value: &mut ByondValue) -> Result<()> {
		let data = self.data().as_ffi();
		let (high, low) = (
			f32::from_bits((data >> 32) as u32),
			f32::from_bits((data & 0xFFFFFFFF) as u32),
		);
		value.write_var(Self::HIGH_VAR, &high.into())?;
		value.write_var(Self::LOW_VAR, &low.into())?;
		Ok(())
	}

	pub fn load(value: &ByondValue) -> Result<Self> {
		let high = value.read_var(Self::HIGH_VAR)?.get_number()?.to_bits();
		let low = value.read_var(Self::LOW_VAR)?.get_number()?.to_bits();
		Ok(KeyData::from_ffi(((high as u64) << 32) | (low as u64)).into())
	}
}
