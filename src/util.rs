use std::f64::consts::PI;
use macroquad::prelude::{screen_height, screen_width};
use crate::structs::*;

pub(crate) fn round_to_place(number: f64, place: u32) -> f64 {
    (number * (10_i32).pow(place) as f64).round() / 10_i32.pow(place) as f64
}

pub(crate) fn screen_to_world(mouse_pos: (f32, f32), scale_factor: f32) -> [f64; 2] {
    let world_x = (mouse_pos.0 - screen_width() / 2.0) / scale_factor;
    let world_y = (mouse_pos.1 - screen_height() / 2.0) / scale_factor;
    [world_x as f64, world_y as f64]
}

pub(crate) fn distance(point_a: [f64; 2], point_b: [f64; 2]) -> f64 {
    ((point_a[0] - point_b[0]).powi(2) + (point_a[1] - point_b[1]).powi(2)).sqrt()
}

pub(crate) fn points_to_horizontal_angle(point_a: [f64; 2], point_b: [f64; 2]) -> f64 {
    let dx = point_a[0] - point_b[0];
    let dy = point_a[1] - point_b[1];
    let mut angle = dy.atan2(dx);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    angle
}

pub(crate) fn vector_to_components(vector: f64, theta: f64) -> [f64; 2] {
    let mut components = [0.0, 0.0];
    let mut direction = [1.0, 1.0];
    let mut ref_angle = 0.0;

    if theta < 1.0 / 2.0 * PI {
        ref_angle = theta;
        direction = [1.0, 1.0]; // 1st quadrant, +x +y
    } else if theta < PI {
        ref_angle = PI - theta;
        direction = [-1.0, 1.0]; // 2nd quadrant, -x +y
    } else if theta < 3.0 / 2.0 * PI {
        ref_angle = theta - PI;
        direction = [-1.0, -1.0]; // 3rd quadrant, -x -y
    } else if theta < 2.0 * PI {
        ref_angle = 2.0 * PI - theta;
        direction = [1.0, -1.0]; // 4th quadrant, +x -y
    }
    components[0] = direction[0] * vector * ref_angle.cos();
    components[1] = direction[1] * vector * ref_angle.sin();
    components
}

pub(crate) fn check_collision(p1: &Particle, p2: &Particle) -> bool {
    let distance = distance(p1.position, p2.position);
    distance <= (p1.radius + p2.radius) as f64
}