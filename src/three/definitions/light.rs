use super::Color;
use wasm_bindgen::prelude::*;

pub trait IntoLight {
    fn into_light(self) -> JsValue;
}

#[wasm_bindgen(module = "/js/three/d.js")]
extern "C" {
    pub type AmbientLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: Color, intensity: f64) -> AmbientLight;
}

impl IntoLight for &AmbientLight {
    fn into_light(self) -> JsValue {
        self.into()
    }
}
