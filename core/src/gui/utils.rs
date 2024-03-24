use super::console;
use crate::{
    scenario::Scenario,
    towers::{
        components::{TowerMarker, TowerPosition},
        config::match_config,
    },
};
use bevy::{ecs::system::SystemState, prelude::*};
use wasm_bindgen::JsValue;

pub fn get_prop(val: &JsValue, key: &str) -> JsValue {
    let key_val = JsValue::from_str(key);
    js_sys::Reflect::get(val, &key_val).unwrap()
}

/**
 * Window coordinates is same as clientX, clientY in JS-land
 *   ie origin at top-left, y-axis points down, x-axis points right
 * https://bevyengine.org/examples-webgpu/3D%20Rendering/3d-viewport-to-world/
 */
pub fn window_to_world_coords(
    world: &mut World,
    pos: (f32, f32),
) -> (f32, f32) {
    let (camera, transform) =
        world.query::<(&Camera, &GlobalTransform)>().single(world);

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let ray = camera
        .viewport_to_world(transform, Vec2::new(pos.0, pos.1))
        .unwrap();

    // Calculate if and where the ray is hitting the ground plane.
    let distance = ray
        .intersect_plane(
            Vec3::new(0., 0., 0.),
            Plane3d::new(Vec3::new(0., 1., 0.)),
        )
        .unwrap();

    let result = ray.get_point(distance);
    (result.x, result.z)
}

pub fn snap_coords(pos: (f32, f32)) -> (i16, i16) {
    (pos.0.round() as i16, pos.1.round() as i16)
}

pub fn can_place_tower(
    world: &mut World,
    top_left: (i16, i16),
    id_tower: u16,
) -> bool {
    let mut state: SystemState<(
        Query<&TowerPosition, With<TowerMarker>>,
        Res<Scenario>,
    )> = SystemState::new(world);

    let (tower_query, scenario) = state.get_mut(world);

    let tower_pos =
        TowerPosition::new(top_left, match_config(id_tower).size);

    for pos in tower_pos.coords {
        for tower in tower_query.iter() {
            if tower.coords.contains(&pos) {
                return false;
            }
        }

        for path in scenario.paths.values() {
            for pt in path.points.iter() {
                if pos == pt.pos {
                    return false;
                }
            }

            if path.buffer_points.contains(&pos) {
                return false;
            }
        }
    }

    true
}
