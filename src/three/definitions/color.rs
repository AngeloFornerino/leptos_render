use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/three/d.js")]
extern "C" {
    pub type Color;

    #[wasm_bindgen(constructor, js_name = "new")]
    pub fn new_with_rgb(rgb: u32) -> Color;

    #[wasm_bindgen(constructor, js_name = "new")]
    pub fn new_with_rgb_components(r: f64, g: f64, b: f64) -> Color;

    #[wasm_bindgen(constructor, js_name = "new")]
    pub fn new_with_str(s: &str) -> Color;
}
