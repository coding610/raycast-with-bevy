use bevy::prelude::*;
use player::*;
use player::raycast::*;
use tile::*;
use systems::*;

pub mod player;
pub mod tile;
mod systems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugin(PlayerPlugin)
            .add_plugin(RaycastPlugin)
            .add_plugin(TilePlugin)
            .add_system(toggle_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Debug, // 2D
    Raycastig // 3D (DEFAULT)
}
