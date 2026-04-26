//! The module which wraps the core library [Vec3] struct into a WebAssembly
//! compatible binding.

use wasm_bindgen::prelude::*;
// use moldyn_core::Vec3;

// TYPESCRIPT!(r#"
// /**
//  * A three-dimensional vector.
//  */
// export interface Vec3 {
//     x: number;
//     y: number;
//     z: number;
// }
// "#);

#[wasm_bindgen(inspectable, js_name = Vec3)]
pub struct Vec3Wrapper {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Vec3Wrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3Wrapper {
        Vec3Wrapper { x, y, z }
    }
}
