use super::providers;
use crate::three::definitions::{start_canvas, PerspectiveCamera, Scene};
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[component]
pub fn Canvas(
    children: Children,
    #[prop(default = 75.0)] fov: f64,
    #[prop(optional)] aspect: f64,
    #[prop(default = 0.1)] near: f64,
    #[prop(default = 1000.0)] far: f64,
    #[prop(default = [0.0, 0.0, 5.0])] position: [f64; 3],
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Canvas>();
    let (scene, set_scene) = create_signal::<Option<Rc<Scene>>>(None);

    provide_context(providers::SceneContext(scene));

    input_ref.on_load(move |element| {
        let element: &web_sys::HtmlCanvasElement = &element;
        let as_html_element = element.unchecked_ref::<web_sys::HtmlElement>();
        let scene = Scene::new();
        let aspect = if aspect == 0.0 {
            let client_width = as_html_element.client_width() as f64;
            let client_height = as_html_element.client_height() as f64;
            client_width / client_height
        } else {
            aspect
        };
        let camera = PerspectiveCamera::new(fov, aspect, near, far);
        camera.position().set_x(position[0]);
        camera.position().set_y(position[1]);
        camera.position().set_z(position[2]);
        let s = start_canvas(as_html_element.into(), camera.into(), scene);
        set_scene.set(Some(Rc::new(s)));
    });

    view! {
        <canvas _ref=input_ref style="width: 100%; height: 100%;">
            {children()}
        </canvas>
    }
}
