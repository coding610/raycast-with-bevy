use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use systems::*;
use player::*;
use wall::*;
use ground::*;

mod systems;
mod player;
mod wall;
mod ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(CorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(GroundPlugin)
        .run();
}
