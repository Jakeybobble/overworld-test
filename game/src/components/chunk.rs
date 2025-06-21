use bevy::prelude::*;
use common::components::chunkdata::ChunkData;

#[derive(Component)]
pub struct ChunkDataHandle(pub Handle<ChunkData>);

#[derive(Component)]
pub struct WorldChunk;

#[derive(Component, Clone, Copy, Deref, DerefMut, Eq, PartialEq, Hash)]
pub struct ChunkSpot(pub IVec2);
impl ChunkSpot {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x,y))
    }
    pub fn path(&self) -> String {
        format!("chunkdata/{},{}/data.ron", self.x, self.y)
    }
}

#[derive(Component)]
pub struct ChunkLoader {
    pub range: u32
}
impl Default for ChunkLoader {
    fn default() -> Self {
        Self { range: 1 }
    }
}

#[derive(Resource, Deref)]
pub struct ExistingChunkData(pub Vec<ChunkSpot>);