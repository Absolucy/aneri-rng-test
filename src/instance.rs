// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use self::dispatcher::RngDispatcher;
use crate::ByondSlotKey;
use byondapi::value::ByondValue;
use eyre::Result;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use slotmap::SlotMap;

const DEFAULT_CAPACITY: usize = 16;

static INSTANCES: Lazy<Mutex<SlotMap<ByondSlotKey, RngDispatcher>>> =
	Lazy::new(|| Mutex::new(SlotMap::with_capacity_and_key(DEFAULT_CAPACITY)));

pub(crate) fn free_instances() {
	if let Some(instances) = Lazy::get(&INSTANCES) {
		let mut instances = instances.lock();
		if instances.capacity() > DEFAULT_CAPACITY {
			// Don't use clear(), so we reclaim memory.
			*instances = SlotMap::with_capacity_and_key(DEFAULT_CAPACITY);
		} else {
			// If we're at the default capacity, it's a waste of time to reallocate.
			instances.clear();
		}
	}
}

#[byondapi::bind]
pub fn rng_new(mut src: ByondValue, secure: ByondValue, seed: ByondValue) -> Result<ByondValue> {
	let secure = secure.is_true();
	let seed = if seed.is_null() {
		None
	} else {
		Some(seed.get_number()? as u32)
	};
	let rng = if secure {
		RngDispatcher::blake3(seed)
	} else {
		RngDispatcher::wyrand(seed)
	};
	INSTANCES.lock().insert(rng).save(&mut src)?;
	Ok(ByondValue::null())
}

#[byondapi::bind]
pub fn rng_del(src: ByondValue) -> Result<ByondValue> {
	let src = ByondSlotKey::load(&src)?;
	Ok(INSTANCES.lock().remove(src).is_some().into())
}
