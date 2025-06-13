use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use crate::{assets::chunkdata::{ChunkData, ChunkDataLoader}};
mod assets;
mod components;
mod systems;
mod consts;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Terrain/Overworld test".to_string(),
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
        }),
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
        app.add_systems(Update, (draw_chunk_data, draw_player));
    }

    {
        use systems::chunkstuff::*;
        app.add_systems(Update, (process_chunkdata, do_chunk_loading, update_chunk_spot, on_chunk_loaded));
        app.add_observer(load_chunk);
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