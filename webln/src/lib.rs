// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

//! WebLN - Lightning Web Standard
//!
//! <https://webln.guide>

#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub extern crate secp256k1;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

use js_sys::{Array, Function, Object, Promise, Reflect};
use secp256k1::PublicKey;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Window;

const IS_ENABLED: &str = "isEnabled";
const ENABLE: &str = "enable";
const GET_INFO: &str = "getInfo";
const KEYSEND: &str = "keysend";
const MAKE_INVOICE: &str = "makeInvoice";
const SEND_PAYMENT: &str = "sendPayment";
const SEND_MULTI_PAYMENT: &str = "sendMultiPayment";
const SEND_PAYMENT_ASYNC: &str = "sendPaymentAsync";
const SIGN_MESSAGE: &str = "signMessage";
const VERIFY_MESSAGE: &str = "verifyMessage";
const REQUEST: &str = "request";
const LNURL: &str = "lnurl";
const ON: &str = "on";
const OFF: &str = "off";
const GET_BALANCE: &str = "getBalance";

/// WebLN error
#[derive(Debug)]
pub enum Error {
    /// Generic WASM error
    Wasm(String),
    /// Impossible to get window
    NoGlobalWindowObject,
    /// Impossible to get window
    NamespaceNotFound(String),
    /// Object key not found
    ObjectKeyNotFound(String),
    /// Invalid type: expected a string
    TypeMismatch(String),
    /// User rejected
    UserRejected,
    /// Empty invoice
    EmptyInvoice,
    /// Something's gone wrong
    SomethingGoneWrong,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wasm(e) => write!(f, "{e}"),
            Self::NoGlobalWindowObject => write!(f, "No global `window` object"),
            Self::NamespaceNotFound(n) => write!(f, "`{n}` namespace not found"),
            Self::ObjectKeyNotFound(n) => write!(f, "Key `{n}` not found in object"),
            Self::TypeMismatch(e) => write!(f, "Type mismatch: {e}"),
            Self::UserRejected => write!(f, "User rejected"),
            Self::EmptyInvoice => write!(f, "Empty invoice"),
            Self::SomethingGoneWrong => write!(f, "Something's gone wrong"),
        }
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        let error: String = format!("{e:?}");
        if error.contains("User rejected") {
            Self::UserRejected
        } else {
            Self::Wasm(error)
        }
    }
}

/// Get value from object key
fn get_value_by_key(obj: &Object, key: &str) -> Result<JsValue, Error> {
    Reflect::get(obj, &JsValue::from_str(key))
        .map_err(|_| Error::ObjectKeyNotFound(key.to_string()))
}

trait Deserialize: Sized {
    fn deserialize(value: JsValue) -> Result<Self, Error>;
}

impl Deserialize for bool {
    fn deserialize(value: JsValue) -> Result<Self, Error> {
        value
            .as_bool()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a bool")))
    }
}

/// Get Info Node Response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetInfoNode {
    /// Alias
    pub alias: Option<String>,
    /// Hex encoded public key
    pub pubkey: Option<String>,
    /// Color
    pub color: Option<String>,
}

/// Get Info Method Response
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GetInfoMethod {
    IsEnabled,
    Enable,
    GetInfo,
    Keysend,
    MakeInvoice,
    SendPayment,
    SendMultiPayment,
    SendPaymentAsync,
    SignMessage,
    VerifyMessage,
    Request,
    Lnurl,
    On,
    Off,
    GetBalance,
    Other(String),
}

impl From<&str> for GetInfoMethod {
    fn from(method: &str) -> Self {
        match method {
            IS_ENABLED => Self::IsEnabled,
            ENABLE => Self::Enable,
            GET_INFO => Self::GetInfo,
            KEYSEND => Self::Keysend,
            MAKE_INVOICE => Self::MakeInvoice,
            SEND_PAYMENT => Self::SendPayment,
            SEND_MULTI_PAYMENT => Self::SendMultiPayment,
            SEND_PAYMENT_ASYNC => Self::SendPaymentAsync,
            SIGN_MESSAGE => Self::SignMessage,
            VERIFY_MESSAGE => Self::VerifyMessage,
            REQUEST => Self::Request,
            LNURL => Self::Lnurl,
            ON => Self::On,
            OFF => Self::Off,
            GET_BALANCE => Self::GetBalance,
            other => Self::Other(other.to_string()),
        }
    }
}

impl fmt::Display for GetInfoMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IsEnabled => write!(f, "{IS_ENABLED}"),
            Self::Enable => write!(f, "{ENABLE}"),
            Self::GetInfo => write!(f, "{GET_INFO}"),
            Self::Keysend => write!(f, "{KEYSEND}"),
            Self::MakeInvoice => write!(f, "{MAKE_INVOICE}"),
            Self::SendPayment => write!(f, "{SEND_PAYMENT}"),
            Self::SendMultiPayment => write!(f, "{SEND_MULTI_PAYMENT}"),
            Self::SendPaymentAsync => write!(f, "{SEND_PAYMENT_ASYNC}"),
            Self::SignMessage => write!(f, "{SIGN_MESSAGE}"),
            Self::VerifyMessage => write!(f, "{VERIFY_MESSAGE}"),
            Self::Request => write!(f, "{REQUEST}"),
            Self::Lnurl => write!(f, "{LNURL}"),
            Self::On => write!(f, "{ON}"),
            Self::Off => write!(f, "{OFF}"),
            Self::GetBalance => write!(f, "{GET_BALANCE}"),
            Self::Other(other) => write!(f, "{other}"),
        }
    }
}

/// Get Info Response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetInfoResponse {
    /// Node
    pub node: GetInfoNode,
    /// Methods list
    pub methods: Vec<GetInfoMethod>,
}

impl Deserialize for GetInfoResponse {
    fn deserialize(value: JsValue) -> Result<Self, Error> {
        let get_info_obj: Object = value.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;

        let node_obj: Object = get_value_by_key(&get_info_obj, "node")?
            .dyn_into()
            .map_err(|_| Error::SomethingGoneWrong)?;

        // Extract data
        let alias: Option<String> = get_value_by_key(&node_obj, "alias")?.as_string();
        let pubkey: Option<String> = get_value_by_key(&node_obj, "pubkey")?.as_string();
        let color: Option<String> = get_value_by_key(&node_obj, "color")?.as_string();
        let methods_array: Array = get_value_by_key(&get_info_obj, "methods")?.into();
        let methods: Vec<GetInfoMethod> = methods_array
            .into_iter()
            .filter_map(|m| m.as_string())
            .map(|m| GetInfoMethod::from(m.as_str()))
            .collect();

        Ok(Self {
            node: GetInfoNode {
                alias,
                pubkey,
                color,
            },
            methods,
        })
    }
}

/// Keysend args
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeysendArgs {
    /// Public key of the destination node.
    pub destination: PublicKey,
    /// Amount in SAT
    pub amount: u64,
    // TODO: add TLVRegistry enum
    // The key should be a stringified integer from the <https://github.com/satoshisstream/satoshis.stream/blob/main/TLV_registry.md>.
    // The value should be an unencoded, plain string.
    // pub custom: Option<HashMap<String, String>>,
}

/// Send Payment Response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendPaymentResponse {
    /// Preimage
    pub preimage: String,
}

impl Deserialize for SendPaymentResponse {
    fn deserialize(value: JsValue) -> Result<Self, Error> {
        let send_payment_obj: Object = value.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;
        let preimage = get_value_by_key(&send_payment_obj, "preimage")?
            .as_string()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [preimage]")))?;
        Ok(Self { preimage })
    }
}

/// Send Multi Payment Single response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendMultiPaymentSingle {
    /// Payment request
    pub payment_request: String,
    /// Error message
    pub response: SendPaymentResponse,
}

/// Send Multi Payment Error
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendMultiPaymentError {
    /// Payment request
    pub payment_request: String,
    /// Error message
    pub message: String,
}

impl Deserialize for SendMultiPaymentError {
    fn deserialize(value: JsValue) -> Result<Self, Error> {
        let error_obj: Object = value.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;
        let payment_request = get_value_by_key(&error_obj, "paymentRequest")?
            .as_string()
            .ok_or_else(|| {
                Error::TypeMismatch(String::from("expected a string [paymentRequest]"))
            })?;
        let message = get_value_by_key(&error_obj, "message")?
            .as_string()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [message]")))?;
        Ok(Self {
            payment_request,
            message,
        })
    }
}

/// Send Payment Response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendMultiPaymentResponse {
    /// Payments
    pub payments: Vec<SendMultiPaymentSingle>,
    /// Errors  
    pub errors: Vec<SendMultiPaymentError>,
}

impl Deserialize for SendMultiPaymentResponse {
    fn deserialize(value: JsValue) -> Result<Self, Error> {
        let obj: Object = value.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;

        // let js_payments: Array = self
        // .get_value_by_key(&obj, "payments")?
        // .dyn_into()?;
        let js_errors: Array = get_value_by_key(&obj, "errors")?.dyn_into()?;

        // Deserialize errors
        let mut errors: Vec<SendMultiPaymentError> =
            Vec::with_capacity(js_errors.length() as usize);
        for error in js_errors.into_iter() {
            errors.push(SendMultiPaymentError::deserialize(error)?);
        }

        Ok(Self {
            payments: Vec::new(), // TODO
            errors,
        })
    }
}

/// Request invoice args
///
/// **All amounts are denominated in SAT.**
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestInvoiceArgs {
    /// Amount
    pub amount: Option<u64>,
    /// Default amount
    pub default_amount: Option<u64>,
    /// Minimum amount
    pub minimum_amount: Option<u64>,
    /// Maximum amount
    pub maximum_amount: Option<u64>,
    /// Default memo
    pub default_memo: Option<String>,
}

impl RequestInvoiceArgs {
    /// New empty request invoice
    pub fn new() -> Self {
        Self::default()
    }

    /// Set amount
    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set default amount
    pub fn default_amount(mut self, default_amount: u64) -> Self {
        self.default_amount = Some(default_amount);
        self
    }

    /// Set minimum amount
    pub fn minimum_amount(mut self, minimum_amount: u64) -> Self {
        self.minimum_amount = Some(minimum_amount);
        self
    }

    /// Set maximum amount
    pub fn maximum_amount(mut self, maximum_amount: u64) -> Self {
        self.maximum_amount = Some(maximum_amount);
        self
    }

    /// Set default memo
    pub fn default_memo(mut self, default_memo: String) -> Self {
        self.default_memo = Some(default_memo);
        self
    }
}

impl TryFrom<&RequestInvoiceArgs> for Object {
    type Error = Error;

    fn try_from(args: &RequestInvoiceArgs) -> Result<Self, Self::Error> {
        let obj = Self::new();

        if let Some(amount) = args.amount {
            Reflect::set(
                &obj,
                &JsValue::from_str("amount"),
                &amount.to_string().into(),
            )?;
        }

        if let Some(default_amount) = args.default_amount {
            Reflect::set(
                &obj,
                &JsValue::from_str("defaultAmount"),
                &default_amount.to_string().into(),
            )?;
        }

        if let Some(minimum_amount) = args.minimum_amount {
            Reflect::set(
                &obj,
                &JsValue::from_str("minimumAmount"),
                &minimum_amount.to_string().into(),
            )?;
        }

        if let Some(maximum_amount) = args.maximum_amount {
            Reflect::set(
                &obj,
                &JsValue::from_str("maximumAmount"),
                &maximum_amount.to_string().into(),
            )?;
        }

        if let Some(default_memo) = &args.default_memo {
            Reflect::set(
                &obj,
                &JsValue::from_str("defaultMemo"),
                &default_memo.into(),
            )?;
        }

        Ok(obj)
    }
}

/// Request Invoice Response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestInvoiceResponse {
    /// BOLT-11 invoice
    pub payment_request: String,
}

impl Deserialize for RequestInvoiceResponse {
    fn deserialize(value: JsValue) -> Result<Self, Error> {
        let obj: Object = value.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;
        Ok(Self {
            payment_request: get_value_by_key(&obj, "paymentRequest")?
                .as_string()
                .ok_or_else(|| {
                    Error::TypeMismatch(String::from("expected a string [paymentRequest]"))
                })?,
        })
    }
}

/// Sign Message Response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignMessageResponse {
    /// Message
    pub message: String,
    /// Signature
    pub signature: String,
}

impl Deserialize for SignMessageResponse {
    fn deserialize(value: JsValue) -> Result<Self, Error> {
        let obj: Object = value.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;
        let message: String = get_value_by_key(&obj, "message")?
            .as_string()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [message]")))?;
        let signature: String = get_value_by_key(&obj, "signature")?
            .as_string()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [signature]")))?;
        Ok(Self { message, signature })
    }
}

/// Balance Response
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BalanceResponse {
    /// Balance
    pub balance: f64,
    /// Currency
    pub currency: Option<String>,
}

impl Deserialize for BalanceResponse {
    fn deserialize(value: JsValue) -> Result<Self, Error> {
        let balance_response_obj: Object =
            value.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;
        let balance: f64 = get_value_by_key(&balance_response_obj, "balance")?
            .as_f64()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a number [balance]")))?;
        let currency: Option<String> =
            get_value_by_key(&balance_response_obj, "currency")?.as_string();
        Ok(Self { balance, currency })
    }
}

/// WebLN instance
#[derive(Debug, Clone)]
pub struct WebLN {
    /// `window.webln` object
    webln_obj: Object,
}

unsafe impl Send for WebLN {}

unsafe impl Sync for WebLN {}

impl WebLN {
    /// Compose new WebLN instance
    pub fn new() -> Result<Self, Error> {
        let window: Window = web_sys::window().ok_or(Error::NoGlobalWindowObject)?;
        let namespace: JsValue = Reflect::get(&window, &JsValue::from_str("webln"))
            .map_err(|_| Error::NamespaceNotFound(String::from("webln")))?;
        let webln_obj: Object = namespace
            .dyn_into()
            .map_err(|_| Error::NamespaceNotFound(String::from("webln")))?;
        Ok(Self { webln_obj })
    }

    fn get_func(&self, obj: &Object, name: &str) -> Result<Function, Error> {
        let val: JsValue = Reflect::get(obj, &JsValue::from_str(name))
            .map_err(|_| Error::NamespaceNotFound(name.to_string()))?;
        val.dyn_into()
            .map_err(|_| Error::NamespaceNotFound(name.to_string()))
    }

    /// Check if `webln` is enabled without explicitly enabling it through `webln.enable()`
    /// (which may cause a confirmation popup in some providers)
    pub async fn is_enabled(&self) -> Result<bool, Error> {
        let func: Function = self.get_func(&self.webln_obj, IS_ENABLED)?;
        let promise: Promise = Promise::resolve(&func.call0(&self.webln_obj)?);
        let result: JsValue = JsFuture::from(promise).await?;
        bool::deserialize(result)
    }

    /// To begin interacting with WebLN APIs you'll first need to enable the provider.
    /// Calling `webln.enable()` will prompt the user for permission to use the WebLN capabilities of the browser.
    /// After that you are free to call any of the other API methods.
    pub async fn enable(&self) -> Result<(), Error> {
        let func: Function = self.get_func(&self.webln_obj, ENABLE)?;
        let promise: Promise = Promise::resolve(&func.call0(&self.webln_obj)?);
        JsFuture::from(promise).await?;
        Ok(())
    }

    /// Get information about the connected node and what WebLN methods it supports.
    pub async fn get_info(&self) -> Result<GetInfoResponse, Error> {
        let func: Function = self.get_func(&self.webln_obj, GET_INFO)?;
        let promise: Promise = Promise::resolve(&func.call0(&self.webln_obj)?);
        let result: JsValue = JsFuture::from(promise).await?;
        GetInfoResponse::deserialize(result)
    }

    /// Request the user to send a keysend payment.
    /// This is a spontaneous payment that does not require an invoice and only needs a destination public key and and amount.
    pub async fn keysend(&self, args: &KeysendArgs) -> Result<SendPaymentResponse, Error> {
        let func: Function = self.get_func(&self.webln_obj, KEYSEND)?;

        let keysend_obj = Object::new();
        Reflect::set(
            &keysend_obj,
            &JsValue::from_str("destination"),
            &args.destination.to_string().into(),
        )?;
        Reflect::set(
            &keysend_obj,
            &JsValue::from_str("amount"),
            &args.amount.to_string().into(),
        )?;

        let promise: Promise = Promise::resolve(&func.call1(&self.webln_obj, &keysend_obj.into())?);
        let result: JsValue = JsFuture::from(promise).await?;
        SendPaymentResponse::deserialize(result)
    }

    /// Request that the user creates an invoice to be used by the web app
    pub async fn make_invoice(
        &self,
        args: &RequestInvoiceArgs,
    ) -> Result<RequestInvoiceResponse, Error> {
        let func: Function = self.get_func(&self.webln_obj, MAKE_INVOICE)?;
        let request_invoice_obj: Object = args.try_into()?;
        let promise: Promise =
            Promise::resolve(&func.call1(&self.webln_obj, &request_invoice_obj.into())?);
        let result: JsValue = JsFuture::from(promise).await?;
        RequestInvoiceResponse::deserialize(result)
    }

    /// Request that the user sends a payment for an invoice.
    pub async fn send_payment(&self, invoice: &str) -> Result<SendPaymentResponse, Error> {
        // `lightning-invoice` increase too much the WASM binary size
        // For now just check if invoice is not empty
        if invoice.is_empty() {
            return Err(Error::EmptyInvoice);
        }

        let func: Function = self.get_func(&self.webln_obj, SEND_PAYMENT)?;
        let promise: Promise = Promise::resolve(&func.call1(&self.webln_obj, &invoice.into())?);
        let result: JsValue = JsFuture::from(promise).await?;
        SendPaymentResponse::deserialize(result)
    }

    /// Request that the user sends multiple payments.
    pub async fn send_multi_payment<I, S>(
        &self,
        invoices: I,
    ) -> Result<SendMultiPaymentResponse, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let invoices: Array = invoices
            .into_iter()
            .map(|i| JsValue::from_str(i.as_ref()))
            .collect();
        let func: Function = self.get_func(&self.webln_obj, SEND_MULTI_PAYMENT)?;
        let promise: Promise = Promise::resolve(&func.call1(&self.webln_obj, &invoices.into())?);
        let result: JsValue = JsFuture::from(promise).await?;
        SendMultiPaymentResponse::deserialize(result)
    }

    /// Request that the user sends a payment for an invoice.
    /// The payment will only be initiated and will not wait for a preimage to be returned.
    /// This is useful when paying HOLD Invoices. There is no guarantee that the payment will be successfully sent to the receiver.
    /// It's up to the receiver to check whether or not the invoice has been paid.
    pub async fn send_payment_async(&self, invoice: &str) -> Result<(), Error> {
        // `lightning-invoice` increase too much the WASM binary size
        // For now just check if invoice is not empty
        if invoice.is_empty() {
            return Err(Error::EmptyInvoice);
        }

        let func: Function = self.get_func(&self.webln_obj, SEND_PAYMENT_ASYNC)?;
        let promise: Promise = Promise::resolve(&func.call1(&self.webln_obj, &invoice.into())?);
        let result: JsValue = JsFuture::from(promise).await?;

        if !result.is_object() {
            return Err(Error::SomethingGoneWrong);
        }

        Ok(())
    }

    /// Request that the user signs an arbitrary string message.
    pub async fn sign_message(&self, message: &str) -> Result<SignMessageResponse, Error> {
        let func: Function = self.get_func(&self.webln_obj, SIGN_MESSAGE)?;
        let promise: Promise = Promise::resolve(&func.call1(&self.webln_obj, &message.into())?);
        let result: JsValue = JsFuture::from(promise).await?;
        SignMessageResponse::deserialize(result)
    }

    /// Fetch the balance of the current account.
    pub async fn get_balance(&self) -> Result<BalanceResponse, Error> {
        let func: Function = self.get_func(&self.webln_obj, GET_BALANCE)?;
        let promise: Promise = Promise::resolve(&func.call0(&self.webln_obj)?);
        let result: JsValue = JsFuture::from(promise).await?;
        BalanceResponse::deserialize(result)
    }
}
