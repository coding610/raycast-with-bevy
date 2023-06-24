use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::*;
use std::f32::consts::PI;
use crate::game::player::components::*;
use crate::game::resources::*;


pub fn display_raycast(
    player_query: Query<&Player, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ray_resource: Res<RayVars>,
    mut shapes: ResMut<DebugShapes>
) {
    if let Ok(player) = player_query.get_single() {
        // let window = window_query.get_single().unwrap();
        // 
        // let ray_angle = ray_resource.fov / ray_resource.ray_rotation_step; /* CONST */
        //
        // let x_mul = window.width() / total_ray_lenght;
        // let mut x_pos = 0.0; /* LEFT TO RIGHT */
        //
        // let mut ray_color = Color::rgba(255.0, 255.0, 255.0, 255.0); /* begin with plain WHITE and adjust the brightness */
        // for ray in player.rays.iter() {
        //     ray_color.set_a(255.0 * (1.0 / ray.distance));
        // } 

        //
        // let mut ray_color = Color::rgba(255.0, 255.0, 255.0, 255.0);
        // let mut ray_start = Vec3::new(0.0, 0.0, 0.0);
        //
        // for ray in player.rays.iter() {
        //     ray_color.set_a(255.0 * (1.0 / ray.distance));
        //     shapes.rect().position(ray_start).size(Vec2::new(x_increment, window.height())).color(ray_color);
        //     ray_start.x += x_increment;
        // }
    } 
}
