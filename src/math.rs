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
    let inc_x_horizontal: f32;
    let inc_y_horizontal: f32;
    if rotation.to_degrees() == 0.0 || rotation.to_degrees() == 180.0 { // Looking straight
        inc_x_horizontal = 10000.0;
        inc_y_horizontal = 10000.0;
    } else if quad == "++" || quad == "-+" { // UP
        inc_x_horizontal = ((PI / 2.0) - rotation).tan() * (TILESIZE - (transform.y - rtl.y));
        inc_y_horizontal = TILESIZE - (transform.y - rtl.y);
    } else { // DOWN
        inc_x_horizontal = ((PI / 2.0) - rotation).tan() * (rtl.y - transform.y);
        inc_y_horizontal = rtl.y - transform.y;
    }
    let increment_horizontal = Vec2::new(inc_x_horizontal, inc_y_horizontal);

    //------------------------------------VERTICAL-----------------------------------
    let inc_x_vertical: f32;
    let inc_y_vertical: f32;
    if rotation.to_degrees() == 90.0 || rotation.to_degrees() == 270.0 { // Looking straight 
        inc_x_vertical = 10000.0;
        inc_y_vertical = 10000.0;
    } else if quad == "++" || quad == "+-" { // RIGHT
        inc_x_vertical = TILESIZE - (transform.x - rtl.x);
        inc_y_vertical = rotation.tan() * (TILESIZE - (transform.x - rtl.x));
    } else { // LEFT
        inc_x_vertical = rtl.x - transform.x;
        inc_y_vertical = rotation.tan() * (rtl.x - transform.x);
    }

    let increment_vertical = Vec2::new(inc_x_vertical, inc_y_vertical);

    (increment_vertical, increment_horizontal)
}

pub fn get_ray_increment(rotation: f32, quad: &str) -> (Vec2, Vec2) { /* Rotation in radians */
    //------------------------------------HORIZONTAL-----------------------------------
    let mut increment_horizontal = Vec2::splat(0.0);
    if rotation.to_degrees() == 0.0 || rotation.to_degrees() == 180.0 { /* Looking Straight */
        increment_horizontal = Vec2::splat(10000.0);
    } else if quad == "++" || quad == "-+" { /* UP */
        increment_horizontal.x = ((PI / 2.0) - rotation).tan() * TILESIZE;
        increment_horizontal.y = TILESIZE;
    } else { /* DOWN */
        increment_horizontal.x = ((PI / 2.0) - rotation).tan() * -TILESIZE;
        increment_horizontal.y = -TILESIZE;
    }

    //------------------------------------VERTICAL-----------------------------------
    let mut increment_vertical = Vec2::splat(0.0);
    if rotation.to_degrees() == 90.0 || rotation.to_degrees() == 270.0 { /* Looking Straight */
        increment_vertical = Vec2::splat(10000.0);
    } else if quad == "++" || quad == "+-" { /* LEFT */
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
