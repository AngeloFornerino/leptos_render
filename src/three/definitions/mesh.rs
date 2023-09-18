use super::{IntoGeometry, IntoMaterial, Vector3};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/three/d.js")]
extern "C" {
    pub type Mesh;

    #[wasm_bindgen(constructor, js_name = "new")]
    fn new_raw(geo: JsValue, mat: JsValue) -> Mesh;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Mesh) -> Vector3;

    #[wasm_bindgen(method, setter, js_name = "material")]
    fn set_material_raw(this: &Mesh, mat: JsValue);
}

impl Mesh {
    pub fn new(geo: impl IntoGeometry, mat: impl IntoMaterial) -> Mesh {
        let geo_js_value = geo.into_geometry();
        let mat_js_value = mat.into_material();
        Mesh::new_raw(geo_js_value, mat_js_value)
    }

    pub fn set_material(&self, mat: impl IntoMaterial) {
        let mat_js_value = mat.into_material();
        self.set_material_raw(mat_js_value);
    }
}
