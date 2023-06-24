use bevy::prelude::*;
use super::raycast::components::*;


#[derive(Component)]
pub struct Player {
    pub rotation: f32,
    pub velocity: Vec3,
    pub is_colliding: bool,
    pub rays: Vec<PlayerRay>,
}
