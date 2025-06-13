use bevy::prelude::*;

use crate::assets::chunkdata::ChunkData;

#[derive(Component)]
pub struct ChunkDataHandle(pub Handle<ChunkData>);

#[derive(Component)]
pub struct Loaded;

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct ChunkSpot(pub IVec2);
impl ChunkSpot {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x,y))
    }
}

#[derive(Component)]
pub struct DoesLoadChunk;