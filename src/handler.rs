use wasm_bindgen::prelude::*;
use js_sys::Function;

#[wasm_bindgen]
pub struct CacheHandler {
    encryption_closure: Function,
    decryption_closure: Function
}

#[wasm_bindgen]
impl CacheHandler {
    #[wasm_bindgen(constructor)]
    pub fn new(encryption_closure: Function, decryption_closure: Function) -> CacheHandler {
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