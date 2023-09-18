use super::Color;
use wasm_bindgen::prelude::*;

pub trait IntoMaterial {
    fn into_material(self) -> JsValue;
}

#[wasm_bindgen(module = "/js/three/d.js")]
extern "C" {
    pub type MeshStandardMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MeshStandardMaterial;

    #[wasm_bindgen(method, setter, js_name = "color")]
    pub fn set_color(this: &MeshStandardMaterial, color: Color);
}

impl IntoMaterial for &MeshStandardMaterial {
    fn into_material(self) -> JsValue {
        self.into()
    }
}
