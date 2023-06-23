use bevy::prelude::*;

#[derive(Resource)]
pub struct RayVars {
    pub ray_rotation_step: f32,
    pub ray_max_depth: f32,
    pub fov: f32,
}
