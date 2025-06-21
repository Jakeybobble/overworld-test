use bevy::{
    color::palettes::css::{WHITE, YELLOW},
    prelude::*,
};

use crate::{
    components::chunk::{ChunkLoader, ChunkSpot},
    constants::DO_DEBUG_DRAW,
    systems::player::Player,
};

use common::{components::chunkdata::ChunkData, constants::*};

/// Currently: Draws info from mapdata
pub fn draw_chunk_data(mut gizmos: Gizmos, query: Query<(&ChunkData, &ChunkSpot)>) {
    if !DO_DEBUG_DRAW {
        return;
    }
    gizmos.grid(
        Isometry3d::from_rotation(Quat::from_rotation_x(f32::to_radians(90.))),
        UVec2::new(50, 50),
        Vec2::splat(CHUNK_WIDTH),
        Color::linear_rgba(1., 1., 1., 0.1),
    );

    // Draw each vertex
    for (chunk_data, chunk_spot) in query.iter() {
        let origin = Vec3::new(
            (chunk_spot.x as f32) * CHUNK_WIDTH,
            0.,
            (chunk_spot.y as f32) * CHUNK_WIDTH,
        );
        for pos in chunk_data.vec3_heights() {
            gizmos.sphere(
                Isometry3d::from_translation(origin + pos),
                0.1,
                Color::WHITE,
            );
        }
    }
}

/// Draw player and the chunk they are in
pub fn draw_player(mut gizmos: Gizmos, query: Query<&Transform, With<Player>>, time: Res<Time>) {
    for transform in query.iter() {
        gizmos.sphere(
            Isometry3d::from_translation(transform.translation),
            0.5 + f32::sin(time.elapsed().as_secs_f32() * 7.) * 0.05,
            YELLOW,
        );
    }
    // TODO: That entire "the chunk they are in" part
}

pub fn draw_chunk_loader_range(mut gizmos: Gizmos, query: Query<(&ChunkSpot, &ChunkLoader)>) {
    if !DO_DEBUG_DRAW {
        return;
    }
    for (chunk_spot, chunk_loader) in query.iter() {
        let range = 1. + chunk_loader.range as f32 * 2.;
        gizmos.rect(
            Isometry3d::new(
                Vec3::new(
                    (chunk_spot.x as f32 + 0.5) * CHUNK_WIDTH,
                    0.01,
                    (chunk_spot.y as f32 + 0.5) * CHUNK_WIDTH,
                ),
                Quat::from_rotation_x(f32::to_radians(90.)),
            ),
            Vec2::splat(range * CHUNK_WIDTH),
            WHITE,
        );
    }
}
