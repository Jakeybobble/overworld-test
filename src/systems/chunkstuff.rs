use bevy::prelude::*;

use crate::{
    assets::chunkdata::ChunkData,
    components::chunk::{ChunkDataHandle, ChunkSpot, DoesLoadChunk},
    constants::CHUNK_WIDTH,
};

/// Replaces ChunkDataHandle with ChunkData once the asset has loaded
pub fn process_chunkdata(
    mut events: EventReader<AssetEvent<ChunkData>>,
    chunkdata_asset: Res<Assets<ChunkData>>,
    mut commands: Commands,
    query: Query<(Entity, &ChunkDataHandle)>,
) {
    for event in events.read() {
        if let AssetEvent::Added { id } = event {
            let data = chunkdata_asset.get(*id).unwrap();
            for (entity, chunkdata_handle) in query.iter() {
                if *id == chunkdata_handle.0.id() {
                    commands
                        .entity(entity)
                        .insert(data.clone())
                        .remove::<ChunkDataHandle>();
                }
            }
        }
    }
}

// Updates all ChunkSpot's based on their transforms when they move out of a chunk
pub fn update_chunk_spot(mut query: Query<(&mut ChunkSpot, &Transform)>) {
    for (mut chunk_spot, transform) in query.iter_mut() {
        let new_pos = (transform.translation.xz() / CHUNK_WIDTH)
            .floor()
            .as_ivec2();
        if **chunk_spot != new_pos {
            **chunk_spot = new_pos;
        }
    }
}

// Loads chunks around an entity with DoesLoadChunk on ChunkSpot change
pub fn do_chunk_loading(_query: Query<&ChunkSpot, (With<DoesLoadChunk>, Changed<ChunkSpot>)>) {
    // TODO: Make this account for multiple chunk loading entities
}

/// Generate model!
pub fn on_chunk_loaded(
    query: Query<(Entity, &ChunkData), Added<ChunkData>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, chunk_data) in query.iter() {
        let mut entity = commands.entity(entity);

        let mesh = chunk_data.generate_mesh();

        let mesh_handle: Handle<Mesh> = meshes.add(mesh);
        entity.insert((
            Mesh3d(mesh_handle),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/bluemud.png")),
                ..default()
            })),
        ));
    }
}

#[derive(Event)]
pub struct LoadChunk(pub ChunkSpot);
pub fn load_chunk(
    trigger: Trigger<LoadChunk>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // Make sure to design this (or related systems) so that a chunk can't be double-loaded.

    let vec = *trigger.event().0;
    let data: Handle<ChunkData> =
        asset_server.load(format!("mapdata/{},{}/data.ron", vec.x, vec.y));
    commands.spawn((
        ChunkDataHandle(data),
        ChunkSpot::new(vec.x, vec.y),
        Transform::from_xyz(
            (vec.x as f32) * CHUNK_WIDTH,
            0.,
            (vec.y as f32) * CHUNK_WIDTH,
        ),
    ));
}
