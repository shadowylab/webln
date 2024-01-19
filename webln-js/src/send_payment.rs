// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use wasm_bindgen::prelude::*;
use webln::SendPaymentResponse;

#[wasm_bindgen(js_name = SendPaymentResponse)]
pub struct JsSendPaymentResponse {
    inner: SendPaymentResponse,
}

impl From<SendPaymentResponse> for JsSendPaymentResponse {
    fn from(inner: SendPaymentResponse) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class = SendPaymentResponse)]
impl JsSendPaymentResponse {
    #[wasm_bindgen(getter)]
    pub fn preimage(&self) -> String {
        self.inner.preimage.clone()
    }
}
