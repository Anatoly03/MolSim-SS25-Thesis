//! This library provides a WebAssembly interface for the molecular dynamics
//! simulation. Specifically it provides wrappers for the [moldyn_core] module
//! and exposes them to JavaScript using [wasm_bindgen].
//! 
//! To read more about WebAssembly with Rust, read the [wasm-bindgen guide](https://wasm-bindgen.github.io/wasm-bindgen/introduction.html)

use wasm_bindgen::prelude::*;

// TODO continue reading the web assembly book
// https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {name}!"));
}
