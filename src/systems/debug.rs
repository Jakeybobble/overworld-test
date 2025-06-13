use bevy::{color::palettes::css::YELLOW, prelude::*};

use crate::{assets::chunkdata::ChunkData, components::chunk::ChunkSpot, constants::CHUNK_WIDTH, systems::player::Player};

/// Currently: Draws info from mapdata
pub fn draw_chunk_data(mut gizmos: Gizmos, query: Query<(&ChunkData, &ChunkSpot)>) {
    gizmos.grid(
        Isometry3d::from_rotation(Quat::from_rotation_x(f32::to_radians(90.))),
        UVec2::new(50, 50),
        Vec2::splat(CHUNK_WIDTH),
        Color::linear_rgba(1., 1., 1., 0.1),
    );

    // Draw each vertex
    for (chunk_data, chunk_spot) in query.iter() {
        let origin = Vec3::new((chunk_spot.x as f32) * CHUNK_WIDTH, 0., (chunk_spot.y as f32) * CHUNK_WIDTH);
        for pos in chunk_data.vec3_heights() {
            gizmos.sphere(Isometry3d::from_translation(origin + pos), 0.1, Color::WHITE);
        }
    }
}

/// Draw player and the chunk they are in
pub fn draw_player(mut gizmos: Gizmos, query: Query<&Transform, With<Player>>, time: Res<Time>) {
    for transform in query.iter() {
        gizmos.sphere(Isometry3d::from_translation(transform.translation), 0.5 + f32::sin(time.elapsed().as_secs_f32() * 7.) * 0.05, YELLOW);
    }
    // TODO: That entire "the chunk they are in" part
}