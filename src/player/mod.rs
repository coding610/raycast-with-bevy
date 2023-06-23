use bevy::prelude::*;
use systems::*;
use raycast::*;

mod systems;
mod raycast;
pub mod components;
pub mod consts;


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_plugin(RaycastPlugin)
            .add_system(player_movement)
            .add_system(player_collision);
    }
}
