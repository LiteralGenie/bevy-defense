use super::super::components::UnitModel;
use super::super::health_bar::build_health_bar;
use crate::components::DoNotRender;
use bevy::gltf::Gltf;
use bevy::prelude::*;

#[derive(Resource)]

pub struct UnitAssetHandles {
    model_gltf: Handle<Gltf>,
}

#[derive(Component)]

struct NeedsAnimation;

#[derive(Component)]

pub struct AnimationIndices {
    walk: AnimationNodeIndex,
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
    mut units: Query<
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

    for entity in units.iter_mut() {
        let health_bar = commands
            .spawn(build_health_bar(&mut meshes, &mut materials))
            .id();

        let base = commands
            .spawn(SceneBundle {
                scene: model_gltf.scenes[0].clone(),
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

        commands.entity(entity).insert((
            UnitModel {
                root,
                base,
                health_bar,
            },
            NeedsAnimation,
        ));
    }
}

pub fn render_movement_animation(
    units: Query<(Entity, &UnitModel), With<NeedsAnimation>>,
    mut player_query: Query<
        &mut AnimationPlayer,
        Without<Handle<AnimationGraph>>,
    >,
    children_query: Query<&Children>,
    handles: Res<UnitAssetHandles>,
    assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
) {
    let model_gltf = match assets.get(&handles.model_gltf) {
        Some(x) => x,
        None => return,
    };

    for (entity, model) in units.iter() {
        for child in children_query.iter_descendants(model.base) {
            let Ok(mut player) = player_query.get_mut(child) else {
                continue;
            };

            let mut graph = AnimationGraph::new();

            let idx_walk = graph.add_clip(
                model_gltf.named_animations["Wasp_Flying"]
                    .clone_weak(),
                0.15,
                graph.root,
            );

            player.play(idx_walk).repeat();

            player
                .play(
                    graph.add_clip(
                        model_gltf.named_animations["OnDamage"]
                            .clone_weak(),
                        1.0,
                        graph.root,
                    ),
                )
                .repeat();

            let graph_handle = graphs.add(graph);
            commands.entity(child).insert(graph_handle);

            commands
                .entity(child)
                .insert(AnimationIndices { walk: idx_walk });

            commands.entity(entity).remove::<NeedsAnimation>();
        }
    }
}
