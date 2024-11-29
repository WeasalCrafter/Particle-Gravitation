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

pub(crate) fn calculate_kinetic_energy(particles: &Vec<Particle>, delta_t: f64) -> f64 {
    let mut total_kinetic_energy = 0.0;

    for particle in particles.iter() {
        let velocity = [
            (particle.position[0] - particle.previous_position[0]) / delta_t,
            (particle.position[1] - particle.previous_position[1]) / delta_t,
        ];
        let speed_squared = velocity[0].powi(2) + velocity[1].powi(2);
        total_kinetic_energy += 0.5 * particle.mass * speed_squared;
    }

    total_kinetic_energy
}

pub(crate) fn calculate_potential_energy(particles: &Vec<Particle>, g_constant: f64) -> f64 {
    let mut total_potential_energy = 0.0;

    for i in 0..particles.len() {
        for j in (i + 1)..particles.len() {
            let distance = distance(particles[i].position, particles[j].position);
            if distance > 0.0 {
                total_potential_energy += -g_constant * particles[i].mass * particles[j].mass / distance;
            }
        }
    }

    total_potential_energy
}


pub(crate) fn calculate_linear_momentum(particles: &Vec<Particle>, delta_t: f64) -> [f64; 2] {
    let mut total_momentum = [0.0, 0.0];

    for particle in particles.iter() {
        let velocity = [
            (particle.position[0] - particle.previous_position[0]) / delta_t,
            (particle.position[1] - particle.previous_position[1]) / delta_t,
        ];
        total_momentum[0] += particle.mass * velocity[0];
        total_momentum[1] += particle.mass * velocity[1];
    }

    total_momentum
}


pub(crate) fn calculate_angular_momentum(particles: &Vec<Particle>, delta_t: f64) -> f64 {
    let mut total_mass = 0.0;
    let mut center_of_mass = [0.0, 0.0];

    // Calculate the center of mass
    for particle in particles.iter() {
        total_mass += particle.mass;
        center_of_mass[0] += particle.position[0] * particle.mass;
        center_of_mass[1] += particle.position[1] * particle.mass;
    }
    center_of_mass[0] /= total_mass;
    center_of_mass[1] /= total_mass;

    // Calculate angular momentum
    let mut total_angular_momentum = 0.0;

    for particle in particles.iter() {
        let relative_position = [
            particle.position[0] - center_of_mass[0],
            particle.position[1] - center_of_mass[1],
        ];
        let velocity = [
            (particle.position[0] - particle.previous_position[0]) / delta_t,
            (particle.position[1] - particle.previous_position[1]) / delta_t,
        ];

        total_angular_momentum += particle.mass * (relative_position[0] * velocity[1] - relative_position[1] * velocity[0]);
    }

    total_angular_momentum
}


pub(crate) fn calculate_total_energy(particles: &Vec<Particle>, g_constant: f64, delta_t: f64) -> f64 {
    let kinetic_energy = calculate_kinetic_energy(particles, delta_t);
    let potential_energy = calculate_potential_energy(particles, g_constant);

    kinetic_energy + potential_energy
}

