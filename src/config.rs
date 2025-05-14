use wasm_bindgen::prelude::*;
use js_sys::Function;
use crate::utils;

#[wasm_bindgen]
pub struct CacheConfig {
    total_asset_limit: Option<i32>,
    asset_lifetime: JsValue,
    encryption_closure: Function,
    decryption_closure: Function
}

#[wasm_bindgen]
impl CacheConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(total_asset_limit: Option<i32>, asset_lifetime: JsValue) -> CacheConfig {
        let encryption_closure = Closure::once_into_js(|data: JsValue| utils::encrypt(data)).into();
        let decryption_closure = Closure::once_into_js(|data: JsValue| utils::decrypt(data)).into();

        CacheConfig {
            total_asset_limit,
            asset_lifetime,
            encryption_closure,
            decryption_closure
        }
    }

    #[wasm_bindgen(getter)]
    pub fn get_encryption_closure(&self) -> Function {
        self.encryption_closure.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_encryption_closure(&mut self, closure: Function) {
        self.encryption_closure = closure
    }

    #[wasm_bindgen(getter)]
    pub fn get_decryption_closure(&self) -> Function {
        self.decryption_closure.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_decryption_closure(&mut self, closure: Function) {
        self.decryption_closure = closure
    }
}