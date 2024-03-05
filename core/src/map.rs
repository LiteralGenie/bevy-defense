use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
        texture::{ ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor },
    },
};

#[derive(Resource)]
pub struct Map {
    model: Entity,
}

pub fn spawn_map(
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let plane = commands
        .spawn(PbrBundle {
            mesh: meshes.add(create_mesh(29.99)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(create_texture(asset_server)),
                ..default()
            }),
            ..default()
        })
        .id();

    commands.insert_resource(Map {
        model: plane,
    });
}

fn create_texture(asset_server: ResMut<AssetServer>) -> Handle<Image> {
    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..Default::default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    return asset_server.load_with_settings("debug/map_texture.png", settings);
}

#[rustfmt::skip]
fn create_mesh(size: f32) -> Mesh {
    let height = 0.01;

    return Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                // top
                [size, height, -size], // br
                [size, height, size], // tr
                [-size, height, -size], // bl
                [-size, height, size], // tl
                // bot
                [size, -height, -size], // br
                [size, -height, size], // tr
                [-size, -height, -size], // bl
                [-size, -height, size] // tl
            ]
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                // top
                [size / 2.0, -size / 2.0], // br
                [size / 2.0, size / 2.0], // tr
                [-size / 2.0, -size / 2.0], // bl
                [-size / 2.0, size / 2.0], // tl
                // bot
                [size / 2.0, -size / 2.0], // br
                [size / 2.0, size / 2.0], // tr
                [-size / 2.0, -size / 2.0], // bl
                [-size / 2.0, size / 2.0] // tl
            ]
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 8])
        .with_inserted_indices(Indices::U32(vec![
            0,1,2, 2,1,3, // top
            4,6,5, 5,6,7, // bottom
            5,1,4, 4,1,0, // right (+x)
            6,2,7, 7,2,3, // left (-x)
            5,1,7, 7,1,3, // back (+z)
            4,0,6, 6,0,2, // front (-z)
        ]));
}
