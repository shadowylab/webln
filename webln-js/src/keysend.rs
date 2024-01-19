// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use alloc::string::String;
use core::ops::Deref;
use core::str::FromStr;

use wasm_bindgen::prelude::*;
use webln::secp256k1::PublicKey;
use webln::KeysendArgs;

use crate::error::{into_err, Result};

#[wasm_bindgen(js_name = KeysendArgs)]
pub struct JsKeysendArgs {
    inner: KeysendArgs,
}

impl Deref for JsKeysendArgs {
    type Target = KeysendArgs;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[wasm_bindgen(js_class = KeysendArgs)]
impl JsKeysendArgs {
    pub fn new(destination: String, amount: u32) -> Result<JsKeysendArgs> {
        let destination: PublicKey = PublicKey::from_str(&destination).map_err(into_err)?;
        let amount: u64 = amount as u64;
        Ok(Self {
            inner: KeysendArgs {
                destination,
                amount,
            },
        })
    }
}
