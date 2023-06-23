use bevy::prelude::*;


pub struct PlayerRay {
    pub start: Vec3,
    pub end: Vec3,
    pub collision: Vec3,
    pub direction: (Vec3, Vec3),
    pub rotation: f32,
}
