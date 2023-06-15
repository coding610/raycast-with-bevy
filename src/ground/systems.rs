use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::*;


pub const GROUNDTILE_SIZE: f32 = 41.0;

// Startup system
pub fn spawn_ground(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    let mut wall_positions = vec![];

    for pos_x in 0..(window.width() / GROUNDTILE_SIZE + GROUNDTILE_SIZE) as i32 {
        for pos_y in 0..(window.height() / GROUNDTILE_SIZE + GROUNDTILE_SIZE) as i32 {
            wall_positions.push((
                (pos_x as f32) * GROUNDTILE_SIZE + GROUNDTILE_SIZE / 2.0,
                (pos_y as f32) * GROUNDTILE_SIZE + GROUNDTILE_SIZE / 2.0
            ));
        }
    }

    for pos in wall_positions {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.0, pos.1, 0.0),
                texture: assets_server.load("sprites/ground.png"),
                ..default()
            },
            Ground {}
        ));
    }
}