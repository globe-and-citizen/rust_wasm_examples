use wasm_bindgen::prelude::*;
use js_sys::Function;
use crate::utils;


#[wasm_bindgen]
pub struct CacheHandler {
    encryption_closure: Function,
    decryption_closure: Function
}

#[wasm_bindgen]
impl CacheHandler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CacheHandler {
        let encryption_closure = Closure::once_into_js(|data: JsValue| utils::encrypt(data)).into();
        let decryption_closure = Closure::once_into_js(|data: JsValue| utils::decrypt(data)).into();

        CacheHandler {
            encryption_closure,
            decryption_closure
        }
    }

    #[wasm_bindgen]
    pub fn encrypt(&self, data: JsValue) -> JsValue {
        self.encryption_closure.call1(&JsValue::NULL, &data).unwrap()
    }

    #[wasm_bindgen]
    pub fn decrypt(&self, data: JsValue) -> JsValue {
        self.decryption_closure.call1(&JsValue::NULL, &data).unwrap()
    }
}