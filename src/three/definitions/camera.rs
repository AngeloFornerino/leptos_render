use super::Vector3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/three/d.js")]
extern "C" {
    pub type PerspectiveCamera;

    #[wasm_bindgen(constructor)]
    pub fn new(
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
    ) -> PerspectiveCamera;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &PerspectiveCamera) -> Vector3;

    #[wasm_bindgen(method, setter)]
    pub fn set_position(this: &PerspectiveCamera, pos: &Vector3);

    #[wasm_bindgen(method)]
    pub fn look_at(this: &PerspectiveCamera, pos: &Vector3);
}
