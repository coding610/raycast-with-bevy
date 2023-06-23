#![allow(unused_assignments)]
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use super::components::*;
use super::consts::MAX_DEPTH;
use crate::player::components::*;
use crate::tile::components::RayCollide;
use crate::math::*;

pub fn flush_rays(mut player_query: Query<&mut Player, With<Player>>) {
    if let Ok(mut player) = player_query.get_single_mut() {
        player.rays = vec!();
    }
}

// DEBUG
pub fn set_rotation(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::R) {
            player.rotation = 90.0;
            let new_rot = (360.0 - player_transform.rotation.z).abs();
            player_transform.rotate_z(new_rot);
        }
    }
}

pub fn calculate_rays(
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    _collide_query: Query<&Transform, With<RayCollide>>,
    mut _shapes: ResMut<DebugShapes>
) {
    if let Ok((player_transform, mut player)) = player_query.get_single_mut() {
        let quad = get_quad(player.rotation);
        let start_increment = get_ray_start_increment(
            player_transform.translation,
            player.rotation.to_radians(),
            quad
        );
        let increment = get_ray_increment(
            player_transform.translation,
            player.rotation.to_radians()
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

        let mut depth: f32 = 0.0;
        while depth < MAX_DEPTH {
            depth += 1.0;
            ray_vertical.x += increment.0.x;
            ray_vertical.y += increment.0.y;
            ray_horizontal.x += increment.1.x;
            ray_horizontal.y += increment.1.y;
        }

        let mut minimal_ray = ray_vertical;
        if ray_lenght(ray_horizontal) < ray_lenght(ray_vertical) { minimal_ray = ray_horizontal; println!("hori") }
        else { println!("vert"); }

        player.rays.push(
            PlayerRay {
                start: player_transform.translation,
                end: minimal_ray,
                collision: Vec3::splat(0.0),
                direction: (Vec3::splat(0.0), Vec3::splat(0.0)),
                rotation: 0.0,
            }
        );
    }
}


pub fn draw_rays(
    player_query: Query<&Player, With<Player>>,
    mut lines: ResMut<DebugLines>
) {
    if let Ok(player) = player_query.get_single() {
        for ray in player.rays.iter() {
            lines.line(ray.start, ray.end, 0.0);
        }
    }
}