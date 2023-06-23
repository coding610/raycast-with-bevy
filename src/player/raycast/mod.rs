use bevy::prelude::*;
use systems::*;

mod systems;
pub mod components;
pub mod consts;

pub struct RaycastPlugin;
impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            calculate_rays,
            draw_rays,
            flush_rays
        ).chain())
            .add_system(set_rotation);
    }
}
