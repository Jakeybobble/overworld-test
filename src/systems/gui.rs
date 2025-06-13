use bevy::prelude::*;

use crate::components::chunk::{ChunkSpot, DoesLoadChunk};

#[derive(Component)]
pub struct PlayerChunkText;

pub fn spawn_debug_text(mut commands: Commands) {
    commands
        .spawn((Text::new("Player chunk: "),))
        .with_child((PlayerChunkText, TextSpan::default()));
}

pub fn update_debug_text(mut query: Query<&mut TextSpan, With<PlayerChunkText>>, loaded_query: Single<&ChunkSpot, With<DoesLoadChunk>>) {

    let chunk_spot = **loaded_query.into_inner();

    for mut span in query.iter_mut() {
        **span = format!("{},{}", chunk_spot.x, chunk_spot.y);
    }
}
