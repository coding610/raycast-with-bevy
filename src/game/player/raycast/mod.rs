use bevy::prelude::*;
use resources::*;
use systems::*;


pub mod systems;
pub mod components;
pub mod resources;

pub struct RaycastPlugin;
impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            RayVars {ray_rotation_step: 0.5, ray_max_depth: 5.0, fov: 90.0})
            .add_system(change_ray_vars)
            .add_systems((
                calculate_rays,
                shorten_rays,
                draw_rays,
                flush_rays
            ).chain()); /* Make this on chain in the future for better perf */

    }
}
