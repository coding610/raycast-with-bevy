use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::*;
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
        
        let mut pos = Vec3::new(window.width(), 0.0, 0.0); // Down
        let mut size = Vec2::new(0.0, window.height());
        let x_stepsize = window.width() / player.rays.len() as f32;

        let mut ray_color = Color::rgb(0.0, 1.0, 0.0); /* begin with plain WHITE and adjust the brightness */
            
        for ray in player.rays.iter() {
            let mut new_color: f32 = 1.0 - (ray.distance / (ray_resource.ray_max_depth * TILESIZE)); if new_color < 0.0 { new_color = 0.0; }
            ray_color.set_g(new_color);

            size.x = x_stepsize; // Right to left!

            shapes.rect().position(Vec3::new(pos.x + size.x/2.0, pos.y + size.y/2.0, 0.0)).size(Vec2::new(size.x, size.y)).color(ray_color); // Pos in middle
            pos.x -= size.x; // Right to left!!
        } 
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
