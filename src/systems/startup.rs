use std::f32::consts::PI;

use crate::{
    components::chunk::{ChunkLoader, ChunkSpot}, constants::CHUNK_WIDTH, systems::player::Player
};
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));

    commands.spawn((DirectionalLight {
        illuminance: light_consts::lux::OVERCAST_DAY,
        shadows_enabled: true,
        ..default()
    }, Transform::from_rotation(Quat::from_rotation_x(-PI / 4.))));

    // Spawn chunk
    let pos = ChunkSpot::new(0, 0);

    // Spawn player
    commands.spawn((
        Player,
        Transform::from_xyz(CHUNK_WIDTH / 2., 0., CHUNK_WIDTH / 2.),
        pos,
        ChunkLoader::default(),
    ));
}
