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
    _lines: ResMut<DebugLines>,
    mut shapes: ResMut<DebugShapes>
) {
    if let Ok(player) = player_query.get_single() {
        let window = window_query.get_single().unwrap();
        
        let mut pos = Vec3::new(window.width(), PLAYER_HEIGHT, 0.0); // Down
        let mut size = Vec2::new(0.0, SKY_POS);
        let x_stepsize = window.width() / player.rays.len() as f32;

        let mut ray_color = Color::rgb(0.0, 0.0, 0.0);
            
        for ray in player.rays.iter() {
            let new_color: f32 = 1.0 - ((ray.distance) / (ray_resource.ray_max_depth * TILESIZE)); // Make this less bright
            ray_color.set_g(new_color);

            size.x = x_stepsize; // Right to left!

            // shapes.rect().position(Vec3::new(pos.x + size.x/2.0, pos.y + size.y/2.0, 0.0)).size(Vec2::new(size.x, size.y)).color(ray_color); // Pos in middle TODO: update this to lines, and or fill rectdis
            pos.x -= size.x; // Right to left!!
        } 

        // Render ground and sky

    } 
}

// pub fn draw_rect(pos: Vec3, size: Vec2, lines: &mut ResMut<DebugLines>, color: Color) {
//     for x in pos.x as i32..(pos.x + size.x) as i32 {
//         lines.line_colored(
//             Vec3::new( x as f32, pos.y, 0.0 ),
//             Vec3::new( x as f32 + size.x, pos.y + size.y, 0.0 ),
//             0.0,
//             color
//         );
//     }
// }
