use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Asset, TypePath, Debug, Component, Clone)]
pub struct MapData {
    // Ideas for what to have here:
    // - Chunk resolution
    // - Map size (Would resize image)
}