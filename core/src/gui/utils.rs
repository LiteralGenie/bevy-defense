use bevy::{ecs::system::SystemState, prelude::*};
use wasm_bindgen::JsValue;

use crate::{
    scenario::{Path, Scenario},
    towers::components::{TowerMarker, TowerPosition},
};

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

pub fn snap_coords(pos: Vec3) -> (i16, i16) {
    (pos.x.round() as i16, pos.z.round() as i16)
}

pub fn can_place_tower(world: &mut World, pos: (i16, i16)) -> bool {
    let mut state: SystemState<(
        Query<&TowerPosition, With<TowerMarker>>,
        Res<Scenario>,
    )> = SystemState::new(world);

    let (tower_query, scenario) = state.get_mut(world);

    for tower in tower_query.iter() {
        if pos.0 as i16 == tower.x && pos.1 as i16 == tower.z {
            return false;
        }
    }

    for path in scenario.paths.values() {
        for pt in path.points.iter() {
            if pos.0 as i16 == pt.pos.0 && pos.1 as i16 == pt.pos.1 {
                return false;
            }
        }
    }

    true
}
