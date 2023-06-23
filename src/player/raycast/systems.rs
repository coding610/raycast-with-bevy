#![allow(unused_assignments)]
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use super::components::*;
use super::consts::*;
use crate::player::components::*;
use crate::tile::components::RayCollide;
use crate::math::*;

pub fn flush_rays(mut player_query: Query<&mut Player, With<Player>>) {
    if let Ok(mut player) = player_query.get_single_mut() {
        player.rays = vec!();
    }
}

pub fn calculate_rays(
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    collide_query: Query<&Transform, With<RayCollide>>,
    mut lines: ResMut<DebugLines>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((player_transform, mut player)) = player_query.get_single_mut() {
        let color_ = Color::rgb(255.0, 255.0, 255.0);
        let ray_rotation_step: f32 = 1.0;

        let mut ray_rotation: f32 = -FOV/2.0;
        for _rayindex in 0..((FOV * ray_rotation_step) as i32) {
            ray_rotation += ray_rotation_step;

            let quad = get_quad(player.rotation);
            let start_increment = get_ray_start_increment(
                player_transform.translation,
                player.rotation.to_radians(),
                quad
            );
            let increment = get_ray_increment(
                player.rotation.to_radians(),
                quad,
            );
            let mut ray_vertical = Vec3::new(
                player_transform.translation.x + start_increment.0.x,
                player_transform.translation.y + start_increment.0.y,
                0.0
            );
            let mut ray_horizontal = Vec3::new(
                player_transform.translation.x + start_increment.1.x,
                player_transform.translation.y + start_increment.1.y,
                0.0
            );

            let mut collided_vertical = false;
            let mut collided_horizontal = false;
            let mut depth: f32 = 0.0;
            /* NOTE: IF ONE OF THEM COLLIDES: THE OTHER ONE DOES TOO */
            while depth < MAX_DEPTH {
                if tile_collide_ray(ray_vertical, &collide_query) {
                    collided_vertical = true;
                } if tile_collide_ray(ray_horizontal, &collide_query) {
                    collided_horizontal = true;
                }
                depth += 1.0;
                if !collided_vertical {
                    ray_vertical.x += increment.0.x;
                    ray_vertical.y += increment.0.y;
                } if !collided_horizontal {
                    ray_horizontal.x += increment.1.x;
                    ray_horizontal.y += increment.1.y;
                }
            }

            let minimal_ray;
            if ray_lenght(player_transform.translation, ray_horizontal) < ray_lenght(player_transform.translation, ray_vertical) {
                minimal_ray = ray_horizontal;
            } else {
                minimal_ray = ray_vertical;
            }

            let default_ray;
            if collided_vertical && collided_horizontal {
                default_ray = minimal_ray;
            } else if collided_vertical {
                default_ray = ray_vertical;
            } else if collided_horizontal {
                default_ray = ray_horizontal;
            } else {
                default_ray = minimal_ray;
            }

            if keyboard_input.pressed(KeyCode::Q) {
                lines.line_colored(player_transform.translation, ray_vertical, 0.0, Color::rgb(0.0, 0.0, 255.0));
            } else if keyboard_input.pressed(KeyCode::E) {
                lines.line_colored(player_transform.translation, ray_horizontal, 0.0, Color::rgb(255.0, 0.0, 0.0));
            }
            

            player.rays.push(
                PlayerRay {
                    start: player_transform.translation,
                    end: default_ray,
                    collision: Vec3::splat(0.0),
                    direction: (Vec3::splat(0.0), Vec3::splat(0.0)),
                    rotation: 0.0,
                    color: color_
                }
            );
        }
    }
}


pub fn draw_rays(
    player_query: Query<&Player, With<Player>>,
    mut lines: ResMut<DebugLines>
) {
    if let Ok(player) = player_query.get_single() {
        for ray in player.rays.iter() {
            lines.line_colored(ray.start, ray.end, 0.0, ray.color);
        }
    }
}
