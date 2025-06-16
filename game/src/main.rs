use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use common::yalify;

use crate::{assets::chunkdata::{ChunkData, ChunkDataLoader}};
mod assets;
mod components;
mod systems;
mod constants;

fn main() {
    println!("{}", yalify("Starting game!"));

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "🦕 Terrain/Overworld test".to_string(),
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }).set(AssetPlugin {
        file_path: "../assets".to_string(),
        ..default()
    }));

    // Project could be structured way better, but this was convenient.
    // I should definitely try something like bevy-butler, sometime.

    app.init_asset::<ChunkData>();
    app.init_asset_loader::<ChunkDataLoader>();

    app.add_plugins(PanOrbitCameraPlugin);

    app.add_systems(Startup, systems::startup::setup);
    {
        use systems::debug::*;
        app.add_systems(Update, (draw_chunk_data, draw_player, draw_chunk_loader_range));
    }

    {
        use systems::chunkstuff::*;
        use components::chunk::*;

        app.add_systems(Update, (process_chunkdata, handle_loading_unloading, update_chunk_spot, on_chunk_loaded));
        app.add_observer(load_chunk);

        // TODO: Loop through all chunkdata in its folder to build this...
        app.insert_resource(ExistingChunkData(vec![ChunkSpot::new(0,0), ChunkSpot::new(0,1), ChunkSpot::new(0,2)]));
    }

    {
        use systems::player::*;
        app.add_systems(Update, update_player_position);
    }

    {
        use systems::gui::*;
        app.add_systems(Startup, spawn_debug_text);
        app.add_systems(Update, update_debug_text);
    }

    app.run();
}