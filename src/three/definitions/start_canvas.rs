use super::{PerspectiveCamera, Scene};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/three/helpers/canvas.js")]
extern "C" {
    #[wasm_bindgen(js_name = "startCanvas")]
    pub fn start_canvas(
        canvas: JsValue,
        camera: PerspectiveCamera,
        scene: Scene,
    ) -> Scene;
}
