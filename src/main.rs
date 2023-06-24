use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_prototype_debug_lines::*;
use systems::CorePlugin;
use game::GamePlugin;

mod systems;
mod math;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_system(close_on_esc)
        .add_plugin(CorePlugin)
        .add_plugin(GamePlugin)
        .run();
}
