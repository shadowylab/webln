// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]
#![allow(clippy::new_without_default)]

use wasm_bindgen::prelude::*;

pub mod error;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
