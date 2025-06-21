use bevy::prelude::*;
use common::yalify;

fn main() {
    println!("{}", yalify("Starting editor!"));
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "🦕 Overworld-test map editor".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: "../assets".to_string(),
                ..default()
            }),
    );

    app.run();
}
