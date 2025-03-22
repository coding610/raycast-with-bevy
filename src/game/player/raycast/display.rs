use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::*;
use super::consts::*;
use crate::game::player::components::*;
use crate::game::resources::*;
use crate::game::tile::consts::*;


pub fn display_raycast(
    player_query: Query<&Player, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ray_resource: Res<RayVars>,
    mut lines: ResMut<DebugLines>,
) {
    if let Ok(player) = player_query.get_single() {
        let window = window_query.get_single().unwrap();
        
        let mut pos = Vec3::new(window.width(), 0.0, 0.0); // Down
        let mut size = Vec2::new(0.0, SKY_POS);
        let x_stepsize = window.width() / player.rays.len() as f32;

        let mut ray_color = Color::rgb(0.7, 0.0, 0.0);
            
        for ray in player.rays.iter() {

            let mut wall_height = (DEFAULT_WALL_HEIGHT * window.height()) / ray.distance; if wall_height > MAX_WALL_HEIGHT {wall_height = MAX_WALL_HEIGHT};
            size.y = wall_height;
            // pos.y = wall_height / 2.0;

            size.x = x_stepsize; // Right to left!

            // shapes.rect().position(Vec3::new(pos.x + size.x/2.0, pos.y + size.y/2.0, 0.0)).size(Vec2::new(size.x, size.y)).color(ray_color); // Pos in middle TODO: update this to lines, and or fill rectdis
            // lines.line_colored(pos, pos + size, 0.0, ray_color);
            draw_rect(pos, size, &mut lines, ray_color);
            pos.x -= size.x; // Right to left!!
        } 
    } 
}

pub fn draw_rect(pos: Vec3, size: Vec2, lines: &mut ResMut<DebugLines>, color: Color) { // Huge improvement
    for x in pos.x as i32..(pos.x + size.x) as i32 {
        lines.line_colored(
            Vec3::new( x as f32, pos.y, 0.0 ),
            Vec3::new( x as f32, pos.y + size.y, 0.0 ),
            0.0,
            color
        );
    }
}
