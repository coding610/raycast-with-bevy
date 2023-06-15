use bevy::prelude::*;
use systems::*;

pub mod systems; // Public because of:    const WALLTILE_SIZE: f32
pub mod components;

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_wall);
    }
}
