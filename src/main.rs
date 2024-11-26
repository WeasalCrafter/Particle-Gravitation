mod models;
use models::{solar_system, earth_moon_system};

use macroquad::prelude::*;
use std::f64::consts::PI;

static G_CONSTANT: f64 = 6.674e-11;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics Simulation".to_owned(),
        window_width: 800,  // Adjust the window width
        window_height: 800, // Adjust the window height
        fullscreen: false,
        ..Default::default()
    }
}

#[derive(Clone)]
struct Particle {
    position: [f64; 2],            // meters
    previous_position: [f64; 2],   // meters
    acceleration: [f64; 2],        // m/s^2
    force: [f64; 2],               // newtons
    mass: f64,                     // kilograms
    name: String,
}

struct Model {
    particles: Vec<Particle>, // List of particles
    delta_t: f64,             // Time step
    scale_factor: f32,        // Scale factor
    g_constant: f64,          // Constant for G
    name: String,             // Model name
}

impl Particle {
    fn new(initial_position: [f64; 2], initial_velocity: [f64; 2], mass: f64, time_step: f64, name: String) -> Self {
        let mut previous_position = [initial_position[0], initial_position[1]];
        previous_position[0] -= initial_velocity[0] * time_step;
        previous_position[1] -= initial_velocity[1] * time_step;

        Particle {
            position: initial_position,
            previous_position,
            acceleration: [0.0, 0.0],
            force: [0.0, 0.0],
            mass,
            name,
        }
    }

    fn update(&mut self, delta_t: f64) {
        self.acceleration[0] = self.force[0] / self.mass;
        self.acceleration[1] = self.force[1] / self.mass;

        let mut new_position = [0.0, 0.0];
        new_position[0] = 2.0 * self.position[0] - self.previous_position[0] + self.acceleration[0] * delta_t.powi(2);
        new_position[1] = 2.0 * self.position[1] - self.previous_position[1] + self.acceleration[1] * delta_t.powi(2);

        self.previous_position = self.position;
        self.position = new_position;
    }
}

fn distance(point_a: [f64; 2], point_b: [f64; 2]) -> f64 {
    ((point_a[0] - point_b[0]).powi(2) + (point_a[1] - point_b[1]).powi(2)).sqrt()
}

fn points_to_horizontal_angle(point_a: [f64; 2], point_b: [f64; 2]) -> f64 {
    let dx = point_a[0] - point_b[0];
    let dy = point_a[1] - point_b[1];
    let mut angle = dy.atan2(dx);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    angle
}

fn vector_to_components(vector: f64, theta: f64) -> [f64; 2] {
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

#[macroquad::main(window_conf)]
async fn main() {
    let mut selected_model: Model = solar_system();
    let mut dt = selected_model.delta_t;
    let mut scale_factor = selected_model.scale_factor;
    let mut particles = selected_model.particles.clone();
    let mut g_constant = selected_model.g_constant;

    let mut time = 0.0;
    let mut paused = true;

    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Space){ // Pause
            paused = !paused;
        }

        if is_key_pressed(KeyCode::Key1){ // Select model 1
            selected_model = solar_system();
            dt = selected_model.delta_t;
            g_constant = selected_model.g_constant;
            scale_factor = selected_model.scale_factor;
            particles = selected_model.particles.clone();
            time = 0.0;
            paused = true;
        }
        if is_key_pressed(KeyCode::Key2){ // Select model 2
            selected_model = earth_moon_system();
            dt = selected_model.delta_t;
            g_constant = selected_model.g_constant;
            scale_factor = selected_model.scale_factor;
            particles = selected_model.particles.clone();
            time = 0.0;
            paused = true;
        }

        if is_key_pressed(KeyCode::R){ // Reset
            time = 0.0;
            particles = selected_model.particles.clone();
            paused = true;
        }

        if is_key_pressed(KeyCode::RightBracket){ // Zoom in
            scale_factor *= 1.1;
        }
        if is_key_pressed(KeyCode::LeftBracket){ // Zoom out
            scale_factor *= 0.9;
        }

        // Update particles and compute forces
        if !paused {
            for i in 0..particles.len() {
                particles[i].force = [0.0, 0.0];
                for j in 0..particles.len() {
                    if i != j {
                        let dis = distance(particles[i].position, particles[j].position);
                        let angle = points_to_horizontal_angle(particles[j].position, particles[i].position);

                        let epsilon: f64 = 1.0; // Softening constant
                        let g_force = (g_constant * particles[i].mass * particles[j].mass) / (dis.powi(2) + epsilon.powi(2));
                        let g_components = vector_to_components(g_force, angle);

                        particles[i].force[0] += g_components[0];
                        particles[i].force[1] += g_components[1];
                    }
                }
                particles[i].update(dt);
            }
            time += dt;
        }
        draw_text(
            &format!("Selected Model: {:#}", selected_model.name),
            20.0,
            20.0,
            16.0,
            YELLOW,
        );
        draw_text(
            &format!("{} frames elapsed", time),
            20.0,
            50.0,
            16.0,
            YELLOW,
        );
        // Vertical
        draw_line(
            0.0,
            screen_height()/2.0,
            screen_width(),
            screen_height()/2.0,
            0.5,
            GRAY,
        );
        // Horizontal
        draw_line(
            screen_width()/2.0,
            0.0,
            screen_width()/2.0,
            screen_width(),
            0.5,
            GRAY,
        );
        // Draw particles
        for i in 0..particles.len() {
            let screen_x = particles[i].position[0] as f32 * scale_factor + screen_width() / 2.0; // Center the screen
            let screen_y = particles[i].position[1] as f32 * scale_factor + screen_height() / 2.0;
            draw_circle(screen_x, screen_y, 5.0, WHITE); // Draw particle
            draw_text(
                &format!("{}", particles[i].name),
                screen_x + 10.0,
                screen_y + 10.0,
                16.0,
                GREEN,
            );
        }
        // Draw frame
        next_frame().await;
    }
}