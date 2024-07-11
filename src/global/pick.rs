// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use byondapi::value::ByondValue;
use eyre::Result;

#[byondapi::bind]
pub fn pick(options: ByondValue, secure: ByondValue) -> Result<ByondValue> {
	let secure = secure.is_true();
	shared::pick(&mut global(secure), options)
}

#[byondapi::bind]
pub fn pick_weighted(options: ByondValue, secure: ByondValue) -> Result<ByondValue> {
	let secure = secure.is_true();
	shared::pick_weighted(&mut global(secure), options)
}
