use bevy::prelude::*;
use resources::*;
use systems::*;
use display::*;
use crate::game::SimulationState;

pub mod systems;
pub mod components;
pub mod resources;
pub mod display;
mod consts;

pub struct RaycastPlugin;
impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            RayVars {ray_rotation_step: 0.05, ray_max_depth: 7.0, fov: 90.0})
            .add_system(change_ray_vars)
            .add_systems((
                calculate_rays,
                shorten_rays,
                draw_rays,
                display_raycast.run_if(in_state(SimulationState::Raycastig)).before(flush_rays),
                flush_rays
            ).chain()); /* Make this on chain in the future for better perf */

    }
}
