#![allow(unused_assignments)]
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::*;
use itertools::izip;
use super::components::*;
use super::resources::*;
use crate::game::player::components::*;
use crate::game::tile::components::RayCollide;
use crate::game::tile::consts::TILESIZE;
use crate::math::*;

pub fn flush_rays(mut player_query: Query<&mut Player, With<Player>>) {
    if let Ok(mut player) = player_query.get_single_mut() {
        player.rays = vec!();
    }
}

pub fn change_ray_vars(
    mut ray_resource: ResMut<RayVars>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if keyboard_input.pressed(KeyCode::I) && ray_resource.ray_rotation_step > 0.05 { ray_resource.ray_rotation_step -= 0.05; }
    if keyboard_input.pressed(KeyCode::O) { ray_resource.ray_rotation_step += 0.05; }
    if keyboard_input.pressed(KeyCode::K) { ray_resource.fov += 5.0; }
    if keyboard_input.pressed(KeyCode::L) { ray_resource.fov -= 5.0; }
    if keyboard_input.pressed(KeyCode::P) { ray_resource.ray_max_depth += 1.0; }
    if keyboard_input.pressed(KeyCode::LBracket) { ray_resource.ray_max_depth -= 1.0; } // KeyCode::Å
}

pub fn calculate_rays(
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    collide_query: Query<&Transform, With<RayCollide>>,
    ray_resource: Res<RayVars>,
) {
    if let Ok((player_transform, mut player)) = player_query.get_single_mut() {
        let color_ = Color::rgb(255.0, 255.0, 255.0);
        let mut ray_rotation: f32 = (-ray_resource.fov/2.0) + player.rotation;
        for _rayindex in 0..((ray_resource.fov * (1.0/ray_resource.ray_rotation_step)) as i32) {
            ray_rotation += ray_resource.ray_rotation_step;
            ray_rotation = adjust_rotation(ray_rotation);

            let quad = get_quad(ray_rotation);
            let start_increment = get_ray_start_increment(
                player_transform.translation,
                ray_rotation.to_radians(),
                quad
            );
            let increment = get_ray_increment(
                ray_rotation.to_radians(),
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
            /* NOTE: IF ONE OF THEM COLLIDES: THE OTHER ONE DOES TOO (eventually) */
            while depth < ray_resource.ray_max_depth {
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

            let default_ray;
            if ray_lenght(player_transform.translation, ray_horizontal) < ray_lenght(player_transform.translation, ray_vertical) {
                default_ray = ray_horizontal;
            } else {
                default_ray = ray_vertical;
            }

            player.rays.push(
                PlayerRay {
                    start: player_transform.translation,
                    end: default_ray,
                    distance: ray_lenght(player_transform.translation, default_ray),
                    rotation: ray_rotation.to_radians(),
                    color: color_,
                }
            );

        }
    }
}


pub fn shorten_rays(
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    ray_resource: ResMut<RayVars>
) {
    if let Ok((transform, mut player)) = player_query.get_single_mut() {
        let max_len = ray_resource.ray_max_depth * TILESIZE;
        for ray in player.rays.iter_mut() {
            if ray_lenght(transform.translation, ray.end) > max_len {
                let ray_angle = ray.rotation;
                let mut new_ray = Vec3::new(transform.translation.x, transform.translation.y, 0.0);
                new_ray.x += ray_angle.cos() * max_len;
                new_ray.y += ray_angle.sin() * max_len;
                ray.end = new_ray;
                ray.distance = ray_lenght(ray.start, new_ray);
            }
        }
    }
}

pub fn draw_rays(
    player_query: Query<&Player, With<Player>>,
    mut lines: ResMut<DebugLines>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(player) = player_query.get_single() {
        for (ray, ray_index) in izip!(player.rays.iter(), 0..player.rays.len()) {
            if keyboard_input.pressed(KeyCode::R) {
                lines.line_colored(ray.start, ray.end, 0.0, Color::RED);
            } if ray_index == 0 && keyboard_input.pressed(KeyCode::T) {
                lines.line_colored(ray.start, ray.end, 0.0, Color::rgb(255.0, 255.0, 0.0));
            }
        }
    }
}
