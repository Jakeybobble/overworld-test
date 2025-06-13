use std::f32::consts::PI;

use crate::{
    components::chunk::{ChunkSpot, DoesLoadChunk},
    systems::{chunkstuff::LoadChunk, player::Player},
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
    commands.trigger(LoadChunk(pos));

    commands.trigger(LoadChunk(ChunkSpot::new(0, 1)));
    commands.trigger(LoadChunk(ChunkSpot::new(0, 2)));

    // Spawn player
    commands.spawn((
        Player,
        Transform::default(),
        ChunkSpot(IVec2::ZERO),
        DoesLoadChunk,
    ));
}
