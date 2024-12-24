// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use alloc::string::String;
use core::ops::Deref;

use wasm_bindgen::prelude::*;
use webln::{RequestInvoiceArgs, RequestInvoiceResponse};

#[wasm_bindgen(js_name = RequestInvoiceArgs)]
pub struct JsRequestInvoiceArgs {
    inner: RequestInvoiceArgs,
}

impl Deref for JsRequestInvoiceArgs {
    type Target = RequestInvoiceArgs;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<RequestInvoiceArgs> for JsRequestInvoiceArgs {
    fn from(inner: RequestInvoiceArgs) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class = RequestInvoiceArgs)]
impl JsRequestInvoiceArgs {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: RequestInvoiceArgs::default(),
        }
    }

    pub fn amount(self, amount: u32) -> Self {
        self.inner.amount(amount as u64).into()
    }

    #[wasm_bindgen(js_name = defaultAmount)]
    pub fn default_amount(self, default_amount: u32) -> Self {
        self.inner.default_amount(default_amount as u64).into()
    }

    #[wasm_bindgen(js_name = minimumAmount)]
    pub fn minimum_amount(self, minimum_amount: u32) -> Self {
        self.inner.minimum_amount(minimum_amount as u64).into()
    }

    #[wasm_bindgen(js_name = maximumAmount)]
    pub fn maximum_amount(self, maximum_amount: u32) -> Self {
        self.inner.maximum_amount(maximum_amount as u64).into()
    }

    #[wasm_bindgen(js_name = defaultMemo)]
    pub fn default_memo(self, default_memo: String) -> Self {
        self.inner.default_memo(default_memo).into()
    }
}

#[wasm_bindgen(js_name = RequestInvoiceResponse)]
pub struct JsRequestInvoiceResponse {
    inner: RequestInvoiceResponse,
}

impl From<RequestInvoiceResponse> for JsRequestInvoiceResponse {
    fn from(inner: RequestInvoiceResponse) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class = RequestInvoiceResponse)]
impl JsRequestInvoiceResponse {
    #[wasm_bindgen(getter, js_name = paymentRequest)]
    pub fn payment_request(&self) -> String {
        self.inner.payment_request.clone()
    }
}
