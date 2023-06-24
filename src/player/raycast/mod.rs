use bevy::prelude::*;
use systems::*;
use resources::*;

mod systems;
pub mod components;
pub mod resources;

pub struct RaycastPlugin;
impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RayVars {ray_rotation_step: 1.0, ray_max_depth: 10.0, fov: 90.0} )
            .add_systems((
            calculate_rays,
            shorten_rays,
            draw_rays,
            flush_rays
        ).chain())
            .add_system(chage_ray_vars);
    }
}
