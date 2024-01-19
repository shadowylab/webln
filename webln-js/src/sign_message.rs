// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use alloc::string::String;

use wasm_bindgen::prelude::*;
use webln::SignMessageResponse;

#[wasm_bindgen(js_name = SignMessageResponse)]
pub struct JsSignMessageResponse {
    inner: SignMessageResponse,
}

impl From<SignMessageResponse> for JsSignMessageResponse {
    fn from(inner: SignMessageResponse) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class = SignMessageResponse)]
impl JsSignMessageResponse {
    pub fn message(&self) -> String {
        self.inner.message.clone()
    }

    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}
