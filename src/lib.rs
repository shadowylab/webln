// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

//! WebLN - Lightning Web Standard

use core::fmt;

use js_sys::{Function, Object, Promise, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Window;

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

    /* /// Get value from object key
    fn get_value_by_key(&self, obj: &Object, key: &str) -> Result<JsValue, Error> {
        Reflect::get(obj, &JsValue::from_str(key))
            .map_err(|_| Error::ObjectKeyNotFound(key.to_string()))
    } */

    /// Check if `webln` is enabled without explicitly enabling it through `webln.enable()` 
    /// (which may cause a confirmation popup in some providers)
    pub async fn is_enabled(&self) -> Result<bool, Error> {
        let func: Function = self.get_func(&self.webln_obj, "isEnabled")?;
        let promise: Promise = Promise::resolve(&func.call0(&self.webln_obj)?);
        let result: JsValue = JsFuture::from(promise).await?;
        result
            .as_bool()
            .ok_or_else(|| Error::TypeMismatch(String::from("expected a bool")))
    }
}
