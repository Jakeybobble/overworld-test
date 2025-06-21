use bevy::prelude::*;

use crate::{components::{chunkdata::ChunkData, mapdata::MapData}, loaders::{chunkdata_loader::ChunkDataLoader, mapdata_loader::MapDataLoader}};

pub mod constants;
pub mod loaders;
pub mod components;

pub fn yalify(string: &str) -> String {
    format!("🦕 Yal says: \"{}\"", string)
}

/// Init all common types and asset loaders
pub fn init(app: &mut App) {
    app.init_asset::<ChunkData>();
    app.init_asset_loader::<ChunkDataLoader>();

    app.init_asset::<MapData>();
    app.init_asset_loader::<MapDataLoader>();
}