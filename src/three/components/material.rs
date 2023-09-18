use super::providers;
use crate::three::definitions::{Color, MeshStandardMaterial};
use leptos::*;
use std::rc::Rc;

#[component]
pub fn Material(
    #[prop(optional, into)] color: Option<MaybeSignal<[f64; 3]>>,
) -> impl IntoView {
    let object3d = use_context::<providers::Object3DContext>().unwrap().0;
    let (mat, set_mat) =
        create_signal::<Option<Rc<MeshStandardMaterial>>>(None);

    create_effect(move |_| {
        if let Some(o) = object3d.get() {
            let mat = MeshStandardMaterial::new();
            o.set_material(&mat);
            set_mat.set(Some(Rc::new(mat)));
        }
    });

    create_effect(move |_| {
        // set the the material to the watchable color
        if let Some(mat) = mat.get() {
            let color = if let Some(color) = color {
                color.get()
            } else {
                [1.0, 1.0, 1.0]
            };
            mat.set_color(Color::new_with_rgb_components(
                color[0], color[1], color[2],
            ));
        }
    });

    view! { <></> }
}
