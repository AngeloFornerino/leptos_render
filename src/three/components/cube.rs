use super::providers;
use crate::three::definitions::{BoxGeometry, Mesh, MeshStandardMaterial};
use leptos::*;
use std::rc::Rc;

#[component]
pub fn Cube(
    children: Children,
    #[prop(default = [0.0,0.0,0.0])] position: [f64; 3],
) -> impl IntoView {
    let scene = use_context::<providers::SceneContext>().unwrap().0;

    let (object3d, set_object3d) = create_signal::<Option<Rc<Mesh>>>(None);

    provide_context(providers::Object3DContext(object3d));

    create_effect(move |_| {
        if let Some(scene) = scene.get() {
            let mat = MeshStandardMaterial::new();
            let geo = BoxGeometry::new();
            let mesh = Mesh::new(&geo, &mat);
            mesh.position().set_x(position[0]);
            mesh.position().set_y(position[1]);
            mesh.position().set_z(position[2]);
            scene.add(&mesh);
            set_object3d.set(Some(Rc::new(mesh)));
        }
    });

    view! { <>{children()}</> }
}
