use wasm_bindgen::prelude::*;
use js_sys::Function;

#[wasm_bindgen]
pub struct CacheConfig {
    total_asset_limit: Option<i32>,
    asset_lifetime: JsValue,
    encryption_closure: Function,
    decryption_closure: Function
}