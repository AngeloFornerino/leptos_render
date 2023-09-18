use wasm_bindgen::prelude::*;

pub trait IntoGeometry {
    fn into_geometry(self) -> JsValue;
}

#[wasm_bindgen(module = "/js/three/d.js")]
extern "C" {
    pub type BoxGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new() -> BoxGeometry;
}

impl IntoGeometry for &BoxGeometry {
    fn into_geometry(self) -> JsValue {
        self.into()
    }
}
