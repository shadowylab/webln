// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

//! WebLN - Lightning Web Standard
//!
//! <https://webln.guide>

#![forbid(unsafe_code)]
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
const SEND_PAYMENT_ASYNC: &str = "sendPaymentAsync";
const SIGN_MESSAGE: &str = "signMessage";

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

impl<S> From<S> for GetInfoMethod
where
    S: AsRef<str>,
{
    fn from(method: S) -> Self {
        match method.as_ref() {
            IS_ENABLED => Self::IsEnabled,
            ENABLE => Self::Enable,
            GET_INFO => Self::GetInfo,
            KEYSEND => Self::Keysend,
            MAKE_INVOICE => Self::MakeInvoice,
            SEND_PAYMENT => Self::SendPayment,
            SEND_PAYMENT_ASYNC => Self::SendPaymentAsync,
            SIGN_MESSAGE => Self::SignMessage,
            "verifyMessage" => Self::VerifyMessage,
            "request" => Self::Request,
            "lnurl" => Self::Lnurl,
            "on" => Self::On,
            "off" => Self::Off,
            "getBalance" => Self::GetBalance,
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
            Self::SendPaymentAsync => write!(f, "{SEND_PAYMENT_ASYNC}"),
            Self::SignMessage => write!(f, "{SIGN_MESSAGE}"),
            Self::VerifyMessage => write!(f, "verifyMessage"),
            Self::Request => write!(f, "request"),
            Self::Lnurl => write!(f, "lnurl"),
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::GetBalance => write!(f, "getBalance"),
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
    pub fn default_memo<S>(mut self, default_memo: S) -> Self
    where
        S: Into<String>,
    {
        self.default_memo = Some(default_memo.into());
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
    pub invoice: String,
}

/// Sign Message Response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignMessageResponse {
    /// Message
    pub message: String,
    /// Signature
    pub signature: String,
}

/// WebLN instance
#[derive(Debug, Clone)]
pub struct WebLN {
    /// `window.webln` object
    webln_obj: Object,
}

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

    fn get_func<S>(&self, obj: &Object, name: S) -> Result<Function, Error>
    where
        S: AsRef<str>,
    {
        let name: &str = name.as_ref();
        let val: JsValue = Reflect::get(obj, &JsValue::from_str(name))
            .map_err(|_| Error::NamespaceNotFound(name.to_string()))?;
        val.dyn_into()
            .map_err(|_| Error::NamespaceNotFound(name.to_string()))
    }

    /// Get value from object key
    fn get_value_by_key(&self, obj: &Object, key: &str) -> Result<JsValue, Error> {
        Reflect::get(obj, &JsValue::from_str(key))
            .map_err(|_| Error::ObjectKeyNotFound(key.to_string()))
    }

    /// Check if `webln` is enabled without explicitly enabling it through `webln.enable()`
    /// (which may cause a confirmation popup in some providers)
    pub async fn is_enabled(&self) -> Result<bool, Error> {
        let func: Function = self.get_func(&self.webln_obj, IS_ENABLED)?;
        let promise: Promise = Promise::resolve(&func.call0(&self.webln_obj)?);
        let result: JsValue = JsFuture::from(promise).await?;
        result
            .as_bool()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a bool")))
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
        let get_info_obj: Object = result.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;

        let node_obj: Object = self
            .get_value_by_key(&get_info_obj, "node")?
            .dyn_into()
            .map_err(|_| Error::SomethingGoneWrong)?;

        // Extract data
        let alias: Option<String> = self.get_value_by_key(&node_obj, "alias")?.as_string();
        let pubkey: Option<String> = self.get_value_by_key(&node_obj, "pubkey")?.as_string();
        let color: Option<String> = self.get_value_by_key(&node_obj, "color")?.as_string();
        let methods_array: Array = self.get_value_by_key(&get_info_obj, "methods")?.into();
        let methods: Vec<GetInfoMethod> = methods_array
            .into_iter()
            .filter_map(|m| m.as_string())
            .map(GetInfoMethod::from)
            .collect();

        Ok(GetInfoResponse {
            node: GetInfoNode {
                alias,
                pubkey,
                color,
            },
            methods,
        })
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
        let send_payment_obj: Object = result.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;

        Ok(SendPaymentResponse {
            preimage: self
                .get_value_by_key(&send_payment_obj, "preimage")?
                .as_string()
                .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [preimage]")))?,
        })
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
        let request_invoice_response_obj: Object =
            result.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;
        Ok(RequestInvoiceResponse {
            invoice: self
                .get_value_by_key(&request_invoice_response_obj, "paymentRequest")?
                .as_string()
                .ok_or_else(|| {
                    Error::TypeMismatch(String::from("expected a string [paymentRequest]"))
                })?,
        })
    }

    /// Request that the user sends a payment for an invoice.
    pub async fn send_payment<S>(&self, invoice: S) -> Result<SendPaymentResponse, Error>
    where
        S: AsRef<str>,
    {
        let invoice: &str = invoice.as_ref();

        // `lightning-invoice` increase too much the WASM binary size
        // For now just check if invoice is not empty
        if invoice.is_empty() {
            return Err(Error::EmptyInvoice);
        }

        let func: Function = self.get_func(&self.webln_obj, SEND_PAYMENT)?;
        let promise: Promise = Promise::resolve(&func.call1(&self.webln_obj, &invoice.into())?);
        let result: JsValue = JsFuture::from(promise).await?;
        let send_payment_obj: Object = result.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;
        Ok(SendPaymentResponse {
            preimage: self
                .get_value_by_key(&send_payment_obj, "preimage")?
                .as_string()
                .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [preimage]")))?,
        })
    }

    /// Request that the user sends a payment for an invoice.
    /// The payment will only be initiated and will not wait for a preimage to be returned.
    /// This is useful when paying HOLD Invoices. There is no guarantee that the payment will be successfully sent to the receiver.
    /// It's up to the receiver to check whether or not the invoice has been paid.
    pub async fn send_payment_async<S>(&self, invoice: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        let invoice: &str = invoice.as_ref();

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
    pub async fn sign_message<S>(&self, message: S) -> Result<SignMessageResponse, Error>
    where
        S: AsRef<str>,
    {
        let message: &str = message.as_ref();
        let func: Function = self.get_func(&self.webln_obj, SIGN_MESSAGE)?;
        let promise: Promise = Promise::resolve(&func.call1(&self.webln_obj, &message.into())?);
        let result: JsValue = JsFuture::from(promise).await?;
        let sign_message_response_obj: Object =
            result.dyn_into().map_err(|_| Error::SomethingGoneWrong)?;

        // Extract data
        let signature: String = self
            .get_value_by_key(&sign_message_response_obj, "signature")?
            .as_string()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [signature]")))?;

        Ok(SignMessageResponse {
            message: message.to_string(),
            signature,
        })
    }
}
