use bevy::prelude::*;
use systems::*;
use line_systems::*;

mod systems;
mod line_systems;
pub mod components;


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(draw_minimal_ray)
            .add_system(player_movement)
            .add_system(player_collision);
    }
}
