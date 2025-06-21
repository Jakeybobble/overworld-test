use bevy::{asset::{io::Reader, AssetLoader, LoadContext}, prelude::*};
use crate::{components::mapdata::MapData, loaders::common::RonLoaderError};


#[derive(Default)]
pub struct MapDataLoader;

impl AssetLoader for MapDataLoader {
    type Asset = MapData;
    type Settings = ();
    type Error = RonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = ron::de::from_bytes::<MapData>(&bytes)?;
        Ok(asset)
    }
}
