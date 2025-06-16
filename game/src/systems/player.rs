use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

/// Update player position with basic keyboard controls
pub fn update_player_position(mut query: Query<&mut Transform, With<Player>>, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    let mut vel = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyD) {
        vel.x += 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        vel.x -= 1.;
    }
    if keys.pressed(KeyCode::KeyW) {
        vel.z -= 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        vel.z += 1.;
    }

    const SPEED: f32 = 10.;

    for mut transform in query.iter_mut() {
        transform.translation += vel * SPEED * time.delta_secs();
    }
}