use super::{IntoLight, Mesh};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/three/d.js")]
extern "C" {
    pub type Scene;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene;

    #[wasm_bindgen(method, js_name = "add")]
    pub fn add_with_mesh(this: &Scene, object: &Mesh);

    #[wasm_bindgen(method, js_name = "add")]
    fn add_with_light_raw(this: &Scene, object: JsValue);
}

impl Scene {
    pub fn add(&self, mesh: &Mesh) {
        self.add_with_mesh(mesh);
    }

    pub fn add_with_light(&self, light: impl IntoLight) {
        self.add_with_light_raw(light.into_light());
    }
}
