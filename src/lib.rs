use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

mod handler;
mod config;
mod utils;

// // The function signatures
// function create(config: CacheConfig | undefined) -> CacheHandler;
#[wasm_bindgen]
pub fn create(cfg: Option<config::CacheConfig>) -> handler::CacheHandler {
    match cfg {
        Some(cache_config) => {
            handler::CacheHandler::new(cache_config.get_encryption_closure(), cache_config.get_decryption_closure())
        }
        None => {
            let encryption_closure = Closure::once_into_js(|data: JsValue| utils::encrypt(data)).into();
            let decryption_closure = Closure::once_into_js(|data: JsValue| utils::decrypt(data)).into();
            handler::CacheHandler::new(encryption_closure, decryption_closure)
        }
    }
}

// // if asset is present, return it if not fetch and store then return the assert
// function check_asset(asset_url: string) -> String;
#[wasm_bindgen]
pub fn check_asset(asset_url: String) -> String {
    // todo handle logic here
    console::log_1(&format!("check_asset is called with {}", asset_url).into());
    "check_asset output".to_string()
}

// function delete_asset(asset_url: string);
#[wasm_bindgen]
pub fn delete_asset(asset_url: String) {
    // todo handle logic here
    console::log_1(&format!("delete_asset is called with {}", asset_url).into());
}
