use bevy::prelude::*;


pub struct PlayerRay {
    pub start: Vec3,
    pub end: Vec3,
    pub rotation: f32,
    pub collided: bool,
    pub color: Color
}
