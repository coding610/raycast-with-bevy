use bevy::prelude::*;
use ground::systems::*;
use wall::systems::*;


pub mod consts;
pub mod components;
pub mod ground;
pub mod wall;

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((
            spawn_ground,
            spawn_wall
        ).chain());
    }
}
