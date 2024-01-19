// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]
#![allow(clippy::new_without_default)]
#![no_std]

extern crate alloc;

use alloc::string::String;
use core::ops::Deref;

use wasm_bindgen::prelude::*;
use webln::WebLN;

pub mod balance;
pub mod error;
pub mod get_info;
pub mod keysend;
pub mod request_invoice;
pub mod send_payment;
pub mod sign_message;

use self::balance::JsBalanceResponse;
use self::error::{into_err, Result};
use self::get_info::JsGetInfoResponse;
use self::keysend::JsKeysendArgs;
use self::request_invoice::{JsRequestInvoiceArgs, JsRequestInvoiceResponse};
use self::send_payment::JsSendPaymentResponse;
use self::sign_message::JsSignMessageResponse;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// WebLN instance
#[wasm_bindgen(js_name = WebLN)]
pub struct JsWebLN {
    inner: WebLN,
}

#[wasm_bindgen(js_class = WebLN)]
impl JsWebLN {
    /// Compose new WebLN instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsWebLN> {
        Ok(Self {
            inner: WebLN::new().map_err(into_err)?,
        })
    }

    /// Check if `webln` is enabled without explicitly enabling it through `webln.enable()`
    /// (which may cause a confirmation popup in some providers)
    #[wasm_bindgen(js_name = isEnabled)]
    pub async fn is_enabled(&self) -> Result<bool> {
        self.inner.is_enabled().await.map_err(into_err)
    }

    /// To begin interacting with WebLN APIs you'll first need to enable the provider.
    /// Calling `webln.enable()` will prompt the user for permission to use the WebLN capabilities of the browser.
    /// After that you are free to call any of the other API methods.
    pub async fn enable(&self) -> Result<()> {
        self.inner.enable().await.map_err(into_err)
    }

    /// Get information about the connected node and what WebLN methods it supports.
    #[wasm_bindgen(js_name = getInfo)]
    pub async fn get_info(&self) -> Result<JsGetInfoResponse> {
        Ok(self.inner.get_info().await.map_err(into_err)?.into())
    }

    /// Request the user to send a keysend payment.
    /// This is a spontaneous payment that does not require an invoice and only needs a destination public key and and amount.
    pub async fn keysend(&self, args: &JsKeysendArgs) -> Result<JsSendPaymentResponse> {
        Ok(self
            .inner
            .keysend(args.deref())
            .await
            .map_err(into_err)?
            .into())
    }

    /// Request that the user creates an invoice to be used by the web app
    #[wasm_bindgen(js_name = makeInvoice)]
    pub async fn make_invoice(
        &self,
        args: &JsRequestInvoiceArgs,
    ) -> Result<JsRequestInvoiceResponse> {
        Ok(self
            .inner
            .make_invoice(args.deref())
            .await
            .map_err(into_err)?
            .into())
    }

    /// Request that the user sends a payment for an invoice.
    #[wasm_bindgen(js_name = sendPayment)]
    pub async fn send_payment(&self, invoice: String) -> Result<JsSendPaymentResponse> {
        Ok(self
            .inner
            .send_payment(invoice)
            .await
            .map_err(into_err)?
            .into())
    }

    /// Request that the user sends a payment for an invoice.
    /// The payment will only be initiated and will not wait for a preimage to be returned.
    /// This is useful when paying HOLD Invoices. There is no guarantee that the payment will be successfully sent to the receiver.
    /// It's up to the receiver to check whether or not the invoice has been paid.
    #[wasm_bindgen(js_name = sendPaymentAsync)]
    pub async fn send_payment_async(&self, invoice: String) -> Result<()> {
        self.inner
            .send_payment_async(invoice)
            .await
            .map_err(into_err)
    }

    /// Request that the user signs an arbitrary string message.
    #[wasm_bindgen(js_name = signMessage)]
    pub async fn sign_message(&self, message: String) -> Result<JsSignMessageResponse> {
        Ok(self
            .inner
            .sign_message(message)
            .await
            .map_err(into_err)?
            .into())
    }

    /// Fetch the balance of the current account.
    #[wasm_bindgen(js_name = getBalance)]
    pub async fn get_balance(&self) -> Result<JsBalanceResponse> {
        Ok(self.inner.get_balance().await.map_err(into_err)?.into())
    }
}
