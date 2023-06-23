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

pub fn collide_tile(box1: Vec3, walls: Query<&Transform, With<RayCollide>>) -> bool {
    todo!();
}

pub fn get_tile(rect: Vec3) -> Vec2 {
    Vec2::new(
        (rect.x / TILESIZE).floor(),
        (rect.y / TILESIZE).floor(),
    )
}

// pub fn ray_lenght(ray_start: Vec3, tile_map: Vec<Vec<usize>>) -> f32 {
//     let lenx = (ray_start.x - ray_end.x).abs();
//     let leny = (ray_start.y - ray_end.y).abs();
//     let result = (lenx.powi(2) + leny.powi(2)).sqrt();
//     result
// }

pub fn adjust_rotation(rotation: f32) -> f32 {
    let ar: f32;
    if rotation >= 0.0 && rotation < 90.0 { ar = rotation; }
    else if rotation >= 90.0 && rotation < 180.0 { ar = 90.0 - rotation }
    else if rotation >= 180.0 && rotation < 270.0 { ar = 180.0 - rotation }
    else if rotation >= 270.0 { ar = 270.0 - rotation }
    else { ar = rotation; }

    ar.to_radians()
}

pub fn get_ray_start_increment(transform: Vec3, rotation: f32, quad: &str) -> (Vec2, Vec2) {
    let rtl = get_tile(transform) * TILESIZE; // Relative tile position


    //------------------------------------HORIZONTAL-----------------------------------
    let inc_x_horizontal: f32;
    let inc_y_horizontal: f32;
    if rotation.to_degrees() == 0.0 || rotation.to_degrees() == 180.0 { // Looking straight
        inc_x_horizontal = 0.0;
        inc_y_horizontal = 0.0;
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
        inc_x_vertical = 0.0;
        inc_y_vertical = 0.0;
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

pub fn get_ray_increment(transform: Vec3, rotation: f32) -> (Vec2, Vec2) {
    //------------------------------------HORIZONTAL-----------------------------------
    let inc_x_horizontal: f32;
    let inc_y_horizontal: f32;
    if rotation.to_degrees() == 0.0 || rotation.to_degrees() == 180.0 { /* Looking Straight */
        inc_x_horizontal = 0.0;
        inc_y_horizontal = 0.0;
    } else { /* UP OR DOWN, DOESNT MATTER */
        inc_x_horizontal = ((PI / 2.0) - rotation).tan() * TILESIZE;
        inc_y_horizontal = TILESIZE;
    }

    let increment_horizontal = Vec2::new(inc_x_horizontal, inc_y_horizontal);


    //------------------------------------VERTICAL-----------------------------------
    let inc_x_vertical: f32;
    let inc_y_vertical: f32;
    if rotation.to_degrees() == 0.0 || rotation.to_degrees() == 180.0 { /* Looking Straight */
        inc_x_vertical = 0.0;
        inc_y_vertical = 0.0;
    } else { /* UP OR DOWN, DOESNT MATTER */
        inc_x_vertical = TILESIZE;
        inc_y_vertical = ((PI / 2.0) - rotation).tan() * TILESIZE;
    }

    let increment_vertical = Vec2::new(inc_x_vertical, inc_y_vertical);

    (increment_vertical, increment_horizontal)
}

pub fn ray_lenght(ray: Vec3) -> f32 {
    (ray.x.powi(2) + ray.y.powi(2)).sqrt()
}
