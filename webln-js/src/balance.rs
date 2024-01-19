// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use alloc::string::String;

use wasm_bindgen::prelude::*;
use webln::BalanceResponse;

#[wasm_bindgen(js_name = BalanceResponse)]
pub struct JsBalanceResponse {
    inner: BalanceResponse,
}

impl From<BalanceResponse> for JsBalanceResponse {
    fn from(inner: BalanceResponse) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class = BalanceResponse)]
impl JsBalanceResponse {
    #[wasm_bindgen(getter)]
    pub fn balance(&self) -> f64 {
        self.inner.balance
    }

    #[wasm_bindgen(getter)]
    pub fn currency(&self) -> Option<String> {
        self.inner.currency.clone()
    }
}
