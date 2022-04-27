use serde::{Deserialize, Serialize}; // consider https://crates.io/crates/serde-wasm-bindgen
use std::usize;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Slot {
	pub(crate) task_id: usize,
	pub(crate) begin: usize,
	pub(crate) end: usize,
}
