use super::super::components::UnitModel;
use super::super::health_bar::build_health_bar;
use crate::components::DoNotRender;
use crate::gui::console::{self, log};
use bevy::gltf::Gltf;
use bevy::prelude::*;

// Based on https://bevy-cheatbook.github.io/3d/gltf.html
#[derive(Resource)]

pub struct UnitAssetHandles {
    model_gltf: Handle<Gltf>,
}

pub fn init_assets(mut commands: Commands, ass: Res<AssetServer>) {
    let model = ass.load::<Gltf>("enemy_pack_gltf/Wasp.glb");

    commands.insert_resource(UnitAssetHandles { model_gltf: model });
}

// @todo: consider moving the model generation into a utility function
//        and refactoring the common component spawning / health bar spawning logic
//        into a single system instead of having a per-unit render system like this
//        (like how towers work)
pub fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    handles: Res<UnitAssetHandles>,
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
        let health_bar = commands
            .spawn(build_health_bar(&mut meshes, &mut materials))
            .id();

        let base = commands
            .spawn(SceneBundle {
                scene: model_gltf.scenes[0].clone_weak(),
                transform: Transform::from_xyz(0.0, 0.25, 0.0)
                    .with_scale(Vec3::splat(0.5)),
                ..default()
            })
            .id();

        let root = commands
            .spawn(SpatialBundle {
                ..Default::default()
            })
            .add_child(base)
            .add_child(health_bar)
            .id();

        commands.entity(entity).insert(UnitModel {
            root,
            base,
            health_bar,
        });
    }
}

// @todo: All models currently reference the same entity / animation player component
//        This probably means we can't play different animations for different units (eg on damage)
//        without spawning a new GLTF handle for each unit which will probably have a hefty perf cost
//        Revisit this in when bevy 0.14 lands, which reworks the animation api anyways
pub fn render_movement_animation(
    units: Query<&UnitModel, With<super::Marker>>,
    mut player_query: Query<&mut AnimationPlayer>,
    children_query: Query<&Children>,
    handles: Res<UnitAssetHandles>,
    assets: Res<Assets<Gltf>>,
) {
    let model_gltf = match assets.get(&handles.model_gltf) {
        Some(x) => x,
        None => return,
    };

    for model in units.iter() {
        for child in children_query.iter_descendants(model.base) {
            let Ok(mut animation_player) =
                player_query.get_mut(child)
            else {
                continue;
            };

            let clip = model_gltf.named_animations["Wasp_Flying"]
                .clone_weak();

            if animation_player.is_playing_clip(&clip) {
                continue;
            }

            animation_player.play(clip).repeat();
            break;
        }
    }
}
