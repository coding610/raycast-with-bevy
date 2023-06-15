use bevy::prelude::*;


#[derive(Component)]
pub struct Player {
    pub rotation: f32,
    pub velocity: Vec3,
    pub is_colliding: bool,
    pub rays: Vec<(Vec3, Vec3)>
}
