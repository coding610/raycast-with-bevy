use bevy::prelude::*;
use player::*;
use player::raycast::*;
use tile::*;

pub mod player;
pub mod tile;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(RaycastPlugin)
            .add_plugin(TilePlugin);
    }
}
