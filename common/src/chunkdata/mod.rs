use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::Indices};
use serde::{Deserialize, Serialize};
use crate::constants::*;

pub mod loader;

#[derive(Serialize, Deserialize, Asset, TypePath, Debug, Component, Clone)]
pub struct ChunkData {
    pub heights: Vec<f32>,
}

impl ChunkData {
    pub fn vec3_heights(&self) -> Vec<Vec3> {
        let spacing = CHUNK_WIDTH / (CHUNK_RESOLUTION - 1) as f32;

        self.heights
            .iter()
            .enumerate()
            .map(|(i, height)| {
                let x = (i % CHUNK_RESOLUTION) as f32 * spacing;
                let z = (i / CHUNK_RESOLUTION) as f32 * spacing;

                Vec3::new(x, *height, z)
            })
            .collect()
    }

    pub fn get_index(&self, x: u32, y: u32) -> u32 {
        x + CHUNK_RESOLUTION as u32 * y
    }

    pub fn generate_mesh(&self) -> Mesh {
        let positions = self.vec3_heights();

        let mut indices: Vec<u32> = vec![];
        for x in 0..CHUNK_RESOLUTION as u32 - 1 {
            for y in 0..CHUNK_RESOLUTION as u32 - 1 {
                indices.push(self.get_index(x, y));
                indices.push(self.get_index(x, y + 1));
                indices.push(self.get_index(x + 1, y));

                indices.push(self.get_index(x, y + 1));
                indices.push(self.get_index(x + 1, y + 1));
                indices.push(self.get_index(x + 1, y));
            }
        }

        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            positions
                .iter()
                .map(|vec| Vec2::new(vec.x / CHUNK_WIDTH, vec.z / CHUNK_WIDTH))
                .collect::<Vec<Vec2>>(),
        )
        .with_inserted_indices(Indices::U32(indices))
        .with_computed_normals()
    }
}