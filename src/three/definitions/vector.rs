use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/three/d.js")]
extern "C" {
    pub type Vector3;

    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector3) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector3) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Vector3, x: f64);

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Vector3, y: f64);

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Vector3, z: f64);
}
