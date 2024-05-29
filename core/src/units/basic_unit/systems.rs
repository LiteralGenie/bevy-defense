use super::super::components::UnitModel;
use super::super::health_bar::build_health_bar;
use crate::components::DoNotRender;
use crate::gui::console;
use bevy::gltf::Gltf;
use bevy::prelude::*;

// @todo: consider moving the model generation into a utility function
//        and refactoring the common component spawning / health bar spawning logic
//        into a single system instead of having a per-unit render system like this
//        (like how towers work)
pub fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    handles: Res<UnitAssets>,
    assets: Res<Assets<Gltf>>,
    units: Query<
        Entity,
        (
            With<super::Marker>,
            Without<UnitModel>,
            Without<DoNotRender>,
        ),
    >,
) {
    let model_gltf = match assets.get(&handles.model_gltf) {
        Some(x) => x,
        None => return,
    };

    for entity in units.iter() {
        let health_bar_model = commands
            .spawn(build_health_bar(&mut meshes, &mut materials))
            .id();

        let model = commands
            .spawn(SceneBundle {
                scene: model_gltf.scenes[0].clone_weak(),
                transform: Transform::from_xyz(0.0, 0.25, 0.0)
                    .with_scale(Vec3::splat(0.5)),
                ..default()
            })
            .id();

        let container = commands
            .spawn(SpatialBundle {
                ..Default::default()
            })
            .add_child(model)
            .add_child(health_bar_model)
            .id();

        commands.entity(entity).insert(UnitModel(container));
    }
}

// Based on https://bevy-cheatbook.github.io/3d/gltf.html
#[derive(Resource)]

pub struct UnitAssets {
    model_gltf: Handle<Gltf>,
}

pub fn init_assets(mut commands: Commands, ass: Res<AssetServer>) {
    let model = ass.load::<Gltf>("enemy_pack_gltf/Wasp.glb");

    commands.insert_resource(UnitAssets { model_gltf: model });
}
