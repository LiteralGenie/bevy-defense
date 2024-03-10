use bevy::prelude::*;
use wasm_bindgen::JsValue;

use crate::towers::components::{TowerMarker, TowerPosition};

use super::console;

pub fn get_prop(val: &JsValue, key: &str) -> JsValue {
    let key_val = JsValue::from_str(key);
    js_sys::Reflect::get(val, &key_val).unwrap()
}

/**
 * Window coordinates is same as clientX, clientY in JS-land
 *   ie origin at top-left, y-axis points down, x-axis points right
 * https://bevyengine.org/examples-webgpu/3D%20Rendering/3d-viewport-to-world/
 */
pub fn window_to_world_coords(world: &mut World, pos: Vec2) -> Vec3 {
    let (camera, transform) =
        world.query::<(&Camera, &GlobalTransform)>().single(world);

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let ray = camera.viewport_to_world(transform, pos).unwrap();

    // Calculate if and where the ray is hitting the ground plane.
    let distance = ray
        .intersect_plane(
            Vec3::new(0., 0., 0.),
            Plane3d::new(Vec3::new(0., 1., 0.)),
        )
        .unwrap();

    ray.get_point(distance)
}

pub fn snap_coords(pos: Vec3) -> Vec3 {
    Vec3::new(pos.x.round(), 0.0, pos.z.round())
}

pub fn can_place_tower(world: &mut World, pos: Vec3) -> bool {
    let mut towers =
        world.query_filtered::<&TowerPosition, With<TowerMarker>>();

    for tower in towers.iter(world) {
        if pos.x as i16 == tower.x && pos.z as i16 == tower.z {
            return false;
        }
    }

    true
}
