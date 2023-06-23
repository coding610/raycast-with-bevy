use bevy::prelude::*;
use std::f32::consts::PI;
use crate::tile::consts::TILESIZE;
use crate::tile::components::RayCollide;


pub fn get_quad(ray_rotation: f32) -> &'static str {
    let mut quad = "";
    if ray_rotation >= 0.0 && ray_rotation < 90.0 { quad = "++"; }
    else if ray_rotation >= 90.0 && ray_rotation < 180.0 { quad = "-+"; }
    else if ray_rotation >= 180.0 && ray_rotation < 270.0 { quad = "--"; }
    else if ray_rotation >= 270.0 { quad = "+-"; }

    quad
}

pub fn tile_collide_ray(
    ray: Vec3,
    collides: &Query<&Transform, With<RayCollide>>,
) -> bool {
    let ray_tile = get_tile(ray);
    let ray_down = get_tile(Vec3::new( ray.x, ray.y - 1.0, 0.0 ));
    let ray_left = get_tile(Vec3::new( ray.x - 1.0, ray.y, 0.0 ));
    for collide in collides.iter() {
        let collide_tile = get_tile(collide.translation);
        if ray_tile == collide_tile || ray_down == collide_tile || ray_left == collide_tile {
            return true;
        }
    }

    return false;
}

pub fn get_tile(rect: Vec3) -> Vec2 {
    Vec2::new(
        (rect.x / TILESIZE).floor(),
        (rect.y / TILESIZE).floor(),
    )
}

pub fn get_ray_start_increment(transform: Vec3, rotation: f32, quad: &str) -> (Vec2, Vec2) {
    let rtl = get_tile(transform) * TILESIZE; // Relative tile position

    //------------------------------------HORIZONTAL-----------------------------------
    let adjusted_rotation = (PI / 2.0) - rotation;
    let mut increment_horizontal = Vec2::splat(0.0);
    if quad == "++" || quad == "-+" { // UP
        increment_horizontal.x = (adjusted_rotation).tan() * (TILESIZE - (transform.y - rtl.y));
        increment_horizontal.y = TILESIZE - (transform.y - rtl.y);
    } else { // DOWN
        increment_horizontal.x = (adjusted_rotation).tan() * (rtl.y - transform.y);
        increment_horizontal.y = rtl.y - transform.y;
    }

    //------------------------------------VERTICAL-----------------------------------
    let mut increment_vertical = Vec2::splat(0.0);
    if quad == "++" || quad == "+-" { // RIGHT
        increment_vertical.x = TILESIZE - (transform.x - rtl.x);
        increment_vertical.y = rotation.tan() * (TILESIZE - (transform.x - rtl.x));
    } else { // LEFT
        increment_vertical.x = rtl.x - transform.x;
        increment_vertical.y = rotation.tan() * (rtl.x - transform.x);
    }

    (increment_vertical, increment_horizontal)
}

pub fn get_ray_increment(rotation: f32, quad: &str) -> (Vec2, Vec2) { /* Rotation in radians */
    //------------------------------------HORIZONTAL-----------------------------------
    let mut increment_horizontal = Vec2::splat(0.0);
    let adjusted_rotation = (PI / 2.0) - rotation;
    if quad == "++" || quad == "-+" { /* UP */
        increment_horizontal.x = (adjusted_rotation).tan() * TILESIZE;
        increment_horizontal.y = TILESIZE;
    } else { /* DOWN */
        increment_horizontal.x = (adjusted_rotation).tan() * -TILESIZE;
        increment_horizontal.y = -TILESIZE;
    }

    //------------------------------------VERTICAL-----------------------------------
    let mut increment_vertical = Vec2::splat(0.0);
    if quad == "++" || quad == "+-" { /* LEFT */
        increment_vertical.x = TILESIZE;
        increment_vertical.y = rotation.tan() * TILESIZE;
    } else { /* RIGHT */
        increment_vertical.x = -TILESIZE;
        increment_vertical.y = rotation.tan() * -TILESIZE;
    }

    (increment_vertical, increment_horizontal)
}

pub fn ray_lenght(start_pos: Vec3, ray: Vec3) -> f32 {
    ((start_pos.x - ray.x).abs().powi(2) + (start_pos.y - ray.y).abs().powi(2)).sqrt()
}

/* broow this in future for poss. better perf. */
pub fn adjust_rotation(rotation: f32) -> f32 { 
    let new_rotation;
    if rotation >= 360.0 {
        new_rotation = (360.0 - rotation).abs();
    } else if rotation < 0.0 {
        new_rotation = 360.0 - rotation.abs();
    } else {
        new_rotation = rotation;
    }

    new_rotation
}
