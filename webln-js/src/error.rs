// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use alloc::string::ToString;

use wasm_bindgen::JsValue;

pub type Result<T, E = JsValue> = core::result::Result<T, E>;

#[inline]
pub fn into_err<E>(error: E) -> JsValue
where
    E: ToString,
{
    JsValue::from_str(&error.to_string())
}
