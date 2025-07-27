//! WASM web application for dadagen
//! 
//! Provides dadagen functionality in web browsers
//! via WebAssembly.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Dadagen WASM is ready.", name)
}
