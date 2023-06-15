use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::sprite::collide_aabb::*;
use super::components::*;
use crate::wall::components::*;
use crate::wall::systems::WALLTILE_SIZE;


pub const PLAYER_SIZE: f32 = 41.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_ROTATING_SPEED: f32 = 5.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: assets_server.load("sprites/player_pixel.png"),
            ..default()
        },
        Player {
            rotation: 135.0,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            is_colliding: false,
            rays: vec!(),
        }
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction.y = player.rotation.to_radians().sin();
            direction.x = player.rotation.to_radians().cos();
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction.y = -player.rotation.to_radians().sin();
            direction.x = -player.rotation.to_radians().cos();
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            player.rotation += PLAYER_ROTATING_SPEED;
            player_transform.rotate_z(PLAYER_ROTATING_SPEED.to_radians());
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            player.rotation -= PLAYER_ROTATING_SPEED;
            player_transform.rotate_z(-PLAYER_ROTATING_SPEED.to_radians());
        }

        if player.rotation > 360.0 {
            player.rotation = (360.0 - player.rotation).abs();
        } else if player.rotation < -360.0 {
            player.rotation = 360.0 - player.rotation.abs();
        }

        // Its already normalized, but who cares...
        if direction.length() > 0.0 { direction = direction.normalize(); }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        player.velocity = direction;
    }
}

pub fn player_collision(
    mut player_query: Query<(&mut Transform, &mut Player), Without<Wall>>,
    wall_query: Query<&Transform, With<Wall>>,
) {
    let player_size = Vec2::new(PLAYER_SIZE, PLAYER_SIZE);
    let wall_size = Vec2::new(WALLTILE_SIZE, WALLTILE_SIZE);

    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        for wall_transform in wall_query.iter() {
            let collision_enum = collide(player_transform.translation, player_size, wall_transform.translation, wall_size);

            if collision_enum.is_some() {
                player.is_colliding = true;
                match collision_enum {
                    Some(Collision::Left) => {
                        player_transform.translation.x = wall_transform.translation.x - (WALLTILE_SIZE / 2.0) - (PLAYER_SIZE / 2.0);
                    }
                    Some(Collision::Right) => {
                        player_transform.translation.x = wall_transform.translation.x + (WALLTILE_SIZE / 2.0) + (PLAYER_SIZE / 2.0);
                    },
                    Some(Collision::Top) => {
                        player_transform.translation.y = wall_transform.translation.y + (WALLTILE_SIZE / 2.0) + (PLAYER_SIZE / 2.0);
                    },
                    Some(Collision::Bottom) => {
                        player_transform.translation.y = wall_transform.translation.y - (WALLTILE_SIZE / 2.0) - (PLAYER_SIZE / 2.0);
                    },
                    Some(Collision::Inside) => {},
                    None => {}
                }
            } else {
                player.is_colliding = false;
            }

        }
    }
}
