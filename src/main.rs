use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use systems::CorePlugin;
use player::PlayerPlugin;
use tile::TilePlugin;

mod systems;
mod player;
mod tile;
mod math;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(CorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TilePlugin)
        .run();
}
