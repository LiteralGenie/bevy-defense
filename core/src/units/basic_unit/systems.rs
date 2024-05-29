use super::super::components::UnitModel;
use super::super::health_bar::build_health_bar;
use crate::components::DoNotRender;
use crate::gui::console;
use bevy::prelude::*;

// @todo: consider moving the model generation into a utility function
//        and refactoring the common component spawning / health bar spawning logic
//        into a single system instead of having a per-unit render system like this
//        (like how towers work)
pub fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<UnitAssets>,
    units: Query<
        Entity,
        (
            With<super::Marker>,
            Without<UnitModel>,
            Without<DoNotRender>,
        ),
    >,
) {
    for entity in units.iter() {
        let health_bar_model = commands
            .spawn(build_health_bar(&mut meshes, &mut materials))
            .id();

        let model = commands
            .spawn(SceneBundle {
                scene: assets.model.clone_weak(),
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

// Based on https://github.com/bevyengine/bevy/blob/release-0.13.2/examples/animation/animated_fox.rs
#[derive(Resource)]

pub struct UnitAssets {
    model: Handle<Scene>,
    animations: Vec<Handle<AnimationClip>>,
}

pub fn init_assets(mut commands: Commands, ass: Res<AssetServer>) {
    let model = ass.load::<Scene>("enemy_pack_gltf/Wasp.glb#Scene0");
    let animations: Vec<Handle<AnimationClip>> =
        vec![ass.load("enemy_pack_gltf/Wasp.glb#Animation0")];

    commands.insert_resource(UnitAssets { model, animations });
}
