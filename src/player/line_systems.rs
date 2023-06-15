use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use super::components::*;


pub fn draw_minimal_ray(
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    mut lines: ResMut<DebugLines>
) {
    if let Ok((player_transform, mut player)) = player_query.get_single_mut() {
        let ray_lenght: f32 = 100.0;
        // let ray_end = Vec3::new(
        //     player_transform.translation.x + player.rotation.to_radians().cos() * ray_lenght,
        //     player_transform.translation.y + player.rotation.to_radians().sin() * ray_lenght,
        //     0.0,
        // );
        let ray_end = Vec3::new(
            player_transform.translation.x + player.velocity.x * ray_lenght,
            player_transform.translation.y + player.velocity.y * ray_lenght,
            0.0,
        );

        println!("ray: {}", ray_end.x);

        let line = (player_transform.translation, ray_end);
        player.rays.push(line);
        lines.line(line.0, line.1, 0.0);
    }
}
