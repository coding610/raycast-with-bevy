use bevy::prelude::*;
use systems::*;

mod systems;
pub mod components;

pub struct GroundPlugin;
impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground);
    }
}
