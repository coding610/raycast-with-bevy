use bevy::prelude::*;
use super::components::*;


pub const WALLTILE_SIZE: f32 = 41.0;

// Startup system
pub fn spawn_wall(
    mut commands: Commands,
    assets_server: Res<AssetServer>
) {
    let wall_positions = vec!(
        ([ (10, 10), (10, 14) ], true),

        ([ (0, 0), (31, 0) ], true),
        ([ (0, 0), (0, 18) ], true),
        ([ (0, 18), (31, 18) ], true),
        ([ (31, 0), (31, 18) ], true),
    );


    for wall_pos in wall_positions {
        if wall_pos.1 {
            let mut adjusted_wall_positions = wall_pos.0;
            for pos in 0..adjusted_wall_positions.len() {
                adjusted_wall_positions[pos].0 *= WALLTILE_SIZE as usize;
                adjusted_wall_positions[pos].1 *= WALLTILE_SIZE as usize;
            }

            let mut multiple_wall_positions: Vec<(usize, usize)> = vec!();
            let box_x_max = adjusted_wall_positions[0].0
                .max(adjusted_wall_positions[1].0);
            let box_x_min = adjusted_wall_positions[0].0
                .min(adjusted_wall_positions[1].0);
            let box_y_max = adjusted_wall_positions[0].1
                .max(adjusted_wall_positions[1].1);
            let box_y_min = adjusted_wall_positions[0].1
                .min(adjusted_wall_positions[1].1);

            for new_box_pos_x in (box_x_min..(box_x_max + WALLTILE_SIZE as usize)).step_by(WALLTILE_SIZE as usize) {
                for new_box_pos_y in (box_y_min..(box_y_max + WALLTILE_SIZE as usize)).step_by(WALLTILE_SIZE as usize) {
                    multiple_wall_positions.push((
                        new_box_pos_x,
                        new_box_pos_y
                    ))
                }
            }

            for box_pos in multiple_wall_positions {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(box_pos.0 as f32, box_pos.1 as f32, 0.0),
                        texture: assets_server.load("sprites/wall-blue.png"),
                        ..default()
                    },
                    Wall {}
                ));
            }
        }
    }
}
