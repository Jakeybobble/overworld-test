use bevy::{asset::{io::Reader, AssetLoader, LoadContext}, prelude::*};
use thiserror::Error;
use crate::chunkdata::ChunkData;


#[derive(Default)]
pub struct ChunkDataLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ChunkDataLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for ChunkDataLoader {
    type Asset = ChunkData;
    type Settings = ();
    type Error = ChunkDataLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = ron::de::from_bytes::<ChunkData>(&bytes)?;
        Ok(asset)
    }
}
