// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

//! WebLN - Lightning Web Standard

pub extern crate secp256k1;

use core::fmt;

use js_sys::{Array, Function, Object, Promise, Reflect};
use secp256k1::PublicKey;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Window;

pub const IS_ENABLED: &str = "isEnabled";
pub const ENABLE: &str = "enable";
pub const GET_INFO: &str = "getInfo";
pub const KEYSEND: &str = "keysend";
// pub const MAKE_INVOICE: &str = "makeInvoice";
pub const SEND_PAYMENT: &str = "sendPayment";

/// WebLN error
#[derive(Debug)]
pub enum Error {
    /// Generic WASM error
    Wasm(JsValue),
    /// Impossible to get window
    NoGlobalWindowObject,
    /// Impossible to get window
    NamespaceNotFound(String),
    /// Object key not found
    ObjectKeyNotFound(String),
    /// Invalid type: expected a string
    TypeMismatch(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wasm(e) => write!(f, "{e:?}"),
            Self::NoGlobalWindowObject => write!(f, "No global `window` object"),
            Self::NamespaceNotFound(n) => write!(f, "`{n}` namespace not found"),
            Self::ObjectKeyNotFound(n) => write!(f, "Key `{n}` not found in object"),
            Self::TypeMismatch(e) => write!(f, "Type mismatch: {e}"),
        }
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::Wasm(e)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetInfoNode {
    pub alias: Option<String>,
    pub pubkey: Option<String>,
    pub color: Option<String>,
}

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
            "makeInvoice" => Self::MakeInvoice,
            SEND_PAYMENT => Self::SendPayment,
            "sendPaymentAsync" => Self::SendPaymentAsync,
            "signMessage" => Self::SignMessage,
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
            Self::MakeInvoice => write!(f, "makeInvoice"),
            Self::SendPayment => write!(f, "{SEND_PAYMENT}"),
            Self::SendPaymentAsync => write!(f, "sendPaymentAsync"),
            Self::SignMessage => write!(f, "signMessage"),
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetInfoResponse {
    pub node: GetInfoNode,
    pub methods: Vec<GetInfoMethod>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendPaymentResponse {
    pub preimage: String,
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
        let get_info_obj: Object = result.dyn_into()?;

        let node_obj: Object = self.get_value_by_key(&get_info_obj, "node")?.dyn_into()?;

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
        let send_payment_obj: Object = result.dyn_into()?;

        Ok(SendPaymentResponse {
            preimage: self
                .get_value_by_key(&send_payment_obj, "preimage")?
                .as_string()
                .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [preimage]")))?,
        })
    }

    // TODO: add `make_invoice`

    // Request that the user sends a payment for an invoice.
    pub async fn send_payment(&self, invoice: String) -> Result<SendPaymentResponse, Error> {
        let func: Function = self.get_func(&self.webln_obj, SEND_PAYMENT)?;
        let promise: Promise = Promise::resolve(&func.call1(&self.webln_obj, &invoice.into())?);
        let result: JsValue = JsFuture::from(promise).await?;
        let send_payment_obj: Object = result.dyn_into()?;
        Ok(SendPaymentResponse {
            preimage: self
                .get_value_by_key(&send_payment_obj, "preimage")?
                .as_string()
                .ok_or_else(|| Error::TypeMismatch(String::from("expected a string [preimage]")))?,
        })
    }
}
