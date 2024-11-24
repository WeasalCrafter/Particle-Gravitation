use macroquad::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::f64::consts::PI;

static PARTICLE_ID_COUNTER: AtomicI32 = AtomicI32::new(1);
static DT: f64 = 500.0; // Time increment

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Simulation".to_owned(),
        window_width: 800,  // Adjust the window width
        window_height: 800, // Adjust the window height
        fullscreen: false,
        ..Default::default()
    }
}

struct Particle {
    id: i32,
    position: [f64; 2],            // meters
    previous_position: [f64; 2],   // meters
    acceleration: [f64; 2],        // m/s^2
    force: [f64; 2],               // newtons
    mass: f64,                     // kilograms
}

impl Particle {
    fn new(initial_position: [f64; 2], initial_velocity: [f64; 2], mass: f64, time_step: f64) -> Self {
        let id = PARTICLE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        let mut previous_position = [initial_position[0], initial_position[1]];
        previous_position[0] -= initial_velocity[0] * time_step;
        previous_position[1] -= initial_velocity[1] * time_step;

        Particle {
            id,
            position: initial_position,
            previous_position,
            acceleration: [0.0, 0.0],
            force: [0.0, 0.0],
            mass,
        }
    }

    fn update(&mut self) {
        self.acceleration[0] = self.force[0] / self.mass;
        self.acceleration[1] = self.force[1] / self.mass;

        let mut new_position = [0.0, 0.0];
        new_position[0] = 2.0 * self.position[0] - self.previous_position[0] + self.acceleration[0] * DT.powi(2);
        new_position[1] = 2.0 * self.position[1] - self.previous_position[1] + self.acceleration[1] * DT.powi(2);

        self.previous_position = self.position;
        self.position = new_position;
    }

    fn destroy(&mut self) {
        self.destroy();
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

fn start() -> Vec<Particle> {
    vec![
        Particle::new([0.0, 0.0], [0.0, 0.0], 597.2, DT),      // Earth
        Particle::new([384.4, 0.0], [0.0, 0.01019], 7.348, DT), // Moon
    ]
}

#[macroquad::main(window_conf)]
async fn main() {
    let g_scaled = 6.674e-5;
    let scale_factor = 0.5;

    // Create particles
    let mut particles = start();

    loop {
        clear_background(BLACK);
        
        if is_key_pressed(KeyCode::R){
            PARTICLE_ID_COUNTER.store(1, Ordering::SeqCst);
            particles = start();
        }

        // Update particles and compute forces
        for i in 0..particles.len() {
            particles[i].force = [0.0, 0.0];
            for j in 0..particles.len() {
                if i != j {
                    let dis = distance(particles[i].position, particles[j].position);
                    let angle = points_to_horizontal_angle(particles[j].position, particles[i].position);

                    let g_force = (g_scaled * particles[i].mass * particles[j].mass) / dis.powi(2);
                    let g_components = vector_to_components(g_force, angle);

                    particles[i].force[0] += g_components[0];
                    particles[i].force[1] += g_components[1];
                }
            }
        }

        for particle in &mut particles {
            particle.update();
        }

        // Draw particles
        for particle in &particles {
            let screen_x = particle.position[0] as f32 * scale_factor + screen_width() / 2.0; // Center the screen
            let screen_y = particle.position[1] as f32 * scale_factor + screen_height() / 2.0;

            draw_circle(screen_x, screen_y, 5.0, WHITE); // Draw particle
            draw_text(
                &format!("Particle {}", particle.id),
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