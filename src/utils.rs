use std::fmt::Display;
use wasm_bindgen::JsError;

pub(crate) fn map_js_err<T: Display>(err: T) -> JsError {
    JsError::new(&format!("{err}"))
}
