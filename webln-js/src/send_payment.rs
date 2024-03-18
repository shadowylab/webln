// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use alloc::string::String;
use alloc::vec::Vec;

use wasm_bindgen::prelude::*;
use webln::{
    SendMultiPaymentError, SendMultiPaymentResponse, SendMultiPaymentSingle, SendPaymentResponse,
};

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

#[wasm_bindgen(js_name = SendMultiPaymentSingle)]
pub struct JsSendMultiPaymentSingle {
    inner: SendMultiPaymentSingle,
}

#[wasm_bindgen(js_class = SendMultiPaymentSingle)]
impl JsSendMultiPaymentSingle {
    #[wasm_bindgen(getter, js_name = paymentRequest)]
    pub fn payment_request(&self) -> String {
        self.inner.payment_request.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn response(&self) -> JsSendPaymentResponse {
        self.inner.response.clone().into()
    }
}

impl From<SendMultiPaymentSingle> for JsSendMultiPaymentSingle {
    fn from(inner: SendMultiPaymentSingle) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_name = SendMultiPaymentError)]
pub struct JsSendMultiPaymentError {
    inner: SendMultiPaymentError,
}

#[wasm_bindgen(js_class = SendMultiPaymentError)]
impl JsSendMultiPaymentError {
    #[wasm_bindgen(getter, js_name = paymentRequest)]
    pub fn payment_request(&self) -> String {
        self.inner.payment_request.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.inner.message.clone()
    }
}

impl From<SendMultiPaymentError> for JsSendMultiPaymentError {
    fn from(inner: SendMultiPaymentError) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_name = SendMultiPaymentResponse)]
pub struct JsSendMultiPaymentResponse {
    inner: SendMultiPaymentResponse,
}

impl From<SendMultiPaymentResponse> for JsSendMultiPaymentResponse {
    fn from(inner: SendMultiPaymentResponse) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class = SendMultiPaymentResponse)]
impl JsSendMultiPaymentResponse {
    #[wasm_bindgen(getter)]
    pub fn payments(&self) -> Vec<JsSendMultiPaymentSingle> {
        self.inner
            .payments
            .iter()
            .cloned()
            .map(|e| e.into())
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> Vec<JsSendMultiPaymentError> {
        self.inner
            .errors
            .iter()
            .cloned()
            .map(|e| e.into())
            .collect()
    }
}
