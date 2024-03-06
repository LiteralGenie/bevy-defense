use bevy::prelude::*;
use wasm_bindgen::JsValue;

pub fn get_prop(val: &JsValue, key: &str) -> JsValue {
    let key_val = JsValue::from_str(key);
    return js_sys::Reflect::get(val, &key_val).unwrap();
}

// https://bevyengine.org/examples-webgpu/3D%20Rendering/3d-viewport-to-world/
pub fn window_to_world_coords(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    pos: Vec2,
) -> Vec3 {
    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let ray =
        camera.viewport_to_world(camera_transform, pos).unwrap();

    // Calculate if and where the ray is hitting the ground plane.
    let distance = ray
        .intersect_plane(
            Vec3::new(0., 0., 0.),
            Plane3d::new(Vec3::new(0., 1., 0.)),
        )
        .unwrap();

    ray.get_point(distance)
}
