use bevy::prelude::*;
use crate::game::tile::consts::TILESIZE;


pub const PLAYER_SIZE: f32 = 41.0 * 4.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_ROTATING_SPEED: f32 = 4.0;
pub const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(TILESIZE * 2.0, TILESIZE * 2.0, 0.0);
pub const PLAYER_STARTING_ROTATION: f32 = 135.0;
