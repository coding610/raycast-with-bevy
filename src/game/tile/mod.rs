use bevy::prelude::*;
use wall::systems::*;
use ground::systems::*;
use crate::game::player::systems::*;

pub mod consts;
pub mod components;
pub mod ground;
pub mod wall;

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground.before(spawn_wall))
            .add_startup_system(spawn_wall.before(spawn_player));
    }
}
