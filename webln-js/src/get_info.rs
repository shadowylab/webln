// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use wasm_bindgen::prelude::*;
use webln::GetInfoResponse;

#[wasm_bindgen(js_name = GetInfoResponse)]
pub struct JsGetInfoResponse {
    inner: GetInfoResponse,
}

impl From<GetInfoResponse> for JsGetInfoResponse {
    fn from(inner: GetInfoResponse) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class = GetInfoResponse)]
impl JsGetInfoResponse {
    pub fn alias(&self) -> Option<String> {
        self.inner.node.alias.clone()
    }

    pub fn pubkey(&self) -> Option<String> {
        self.inner.node.pubkey.clone()
    }

    pub fn color(&self) -> Option<String> {
        self.inner.node.color.clone()
    }

    pub fn methods(&self) -> Vec<String> {
        self.inner.methods.iter().map(|m| m.to_string()).collect()
    }
}
