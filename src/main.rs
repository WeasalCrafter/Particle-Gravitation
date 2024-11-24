use plotly::{Plot, Scatter3D};
use plotly::common::{Title};
use plotly::layout::{Axis};

use std::sync::atomic::{AtomicI32, Ordering};
use std::f64::consts::PI;
static PARTICLE_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

struct Particle {
    id : i32,                       // id number
    position: [f64 ; 2],            // meters
    previous_position: [f64 ; 2],   // meters
    acceleration: [f64 ; 2],        // m/s^2
    force: [f64 ; 2],               // newtons
    mass: f64,                      // kilograms
}

impl Particle {
    fn new(initial_position: [f64 ; 2], initial_velocity: [f64 ; 2], mass: f64, time_step: f64) -> Self {
        // increment the particle count
        let id = PARTICLE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut previous_position: [f64; 2] = [initial_position[0],initial_position[1]];

        previous_position[0] = initial_position[0] - initial_velocity[0] * time_step; // change x position
        previous_position[1] = initial_position[1] - initial_velocity[1] * time_step; // change y position

        Particle {
            id,
            position: initial_position,
            previous_position,
            acceleration: [0.0, 0.0],
            force: [0.0, 0.0],
            mass,
        }
    }

    // update the state of the particle based on its net force and current velocity
    fn update(&mut self, dt: f64) {

        // ensure the force effects the acceleration
        self.acceleration[0] = self.force[0] / self.mass;
        self.acceleration[1] = self.force[1] / self.mass;

        // calculate new positions
        let mut new_position: [f64; 2] = [0.0, 0.0];
        new_position[0] = 2.0 * self.position[0] - self.previous_position[0] + self.acceleration[0] * dt.powi(2);
        new_position[1] = 2.0 * self.position[1] - self.previous_position[1] + self.acceleration[1] * dt.powi(2);

        self.previous_position = self.position;
        self.position = new_position;
    }
}

fn distance(point_a: [f64; 2], point_b: [f64; 2]) -> f64 {
    ((point_a[0] - point_b[0]).powi(2) + (point_a[1] - point_b[1]).powi(2)).sqrt()
}

fn points_to_horizontal_angle(point_a: [f64; 2], point_b: [f64; 2]) -> f64 {
    // Translate point_a relative to point_b (make point_b the origin)
    let dx = point_a[0] - point_b[0];
    let dy = point_a[1] - point_b[1];

    // Calculate the angle using atan2 (dy first, dx second to get the correct quadrant)
    let mut angle = dy.atan2(dx);

    // Ensure the angle is non-negative (range: 0 to 2π)
    if angle < 0.0 {
        angle += 2.0 * PI;
    }

    // Return the angle in radians
    angle
}

fn vector_to_components(vector: f64, theta : f64) -> [f64; 2] {
    // accepts a vector and an angle in radians, converts these two values into x and y components
    let mut components = [0.0, 0.0];
    let mut direction = [1.0,1.0];
    let mut ref_angle = 0.0;

    if theta < 1.0 / 2.0 * PI {
        ref_angle = theta;
        direction = [1.0,1.0]; // 1st quadrant, +x +y
    }else if theta <  PI {
        ref_angle = PI - theta;
        direction = [-1.0,1.0]; // 2nd quadrant, -x +y
    }else if theta < 3.0 / 2.0 * PI {
        ref_angle =  theta - PI;
        direction = [-1.0,-1.0]; // 3rd quadrant, -x -y
    }else if theta < 2.0 * PI {
        ref_angle =  2.0 * PI - theta;
        direction = [1.0,-1.0]; // 4th quadrant, +x -y
    }
    components[0] = direction[0] * vector * ref_angle.cos();
    components[1] = direction[1] * vector * ref_angle.sin();
    components
}

fn main() {
    let dt = 1_000.0;            // time increment
    let tf = 500_000.0;            // final time
    let mut tx = 0.0;         // current time
    let g_scaled = 6.674e-5;

    // create particles
    let mut particles = vec![
        Particle::new([0.0, 0.0], [0.0, 0.0], 597.2, dt), //earth
        Particle::new([384.4, 0.0], [0.0, 0.01019], 7.348, dt), //moon
    ];

    let mut trajectory: Vec<Vec<[f64; 3]>> = vec![vec![]; particles.len()]; // Create one trajectory vector per particle
    // run simulation
    while tx < tf {
        for i in 0..particles.len() {
            for j in 0..particles.len() {
                if i != j {
                    // distance and angle between the two particles
                    let dis = distance(particles[i].position, particles[j].position);
                    let angle = points_to_horizontal_angle(particles[j].position, particles[i].position);

                    // g_force = (G * m1 * m2) / (d^2)
                    let g_force = (g_scaled * particles[i].mass * particles[j].mass) / dis.powi(2);
                    let g_components = vector_to_components(g_force, angle);

                    particles[i].force = g_components;
                }
            }
            trajectory[i].push([tx, particles[i].position[0], particles[i].position[1]]);
            particles[i].update(dt);
        }
        tx += dt;
    }

    for particle in &particles {
        println!(
            "Particle {} final position: x={:.2}m, y={:.2}m",
            particle.id, particle.position[0], particle.position[1]
        );
    }
    println!("{:.2} seconds elapsed", tx);
    println!("-------simulation complete-------");

    let mut plot = Plot::new();

    for i in 0..particles.len() {
        let time: Vec<f64> = trajectory[i].iter().map(|point| point[0]).collect();
        let x: Vec<f64> = trajectory[i].iter().map(|point| point[1]).collect();
        let y: Vec<f64> = trajectory[i].iter().map(|point| point[2]).collect();
        let id = particles[i].id;

        let trace = Scatter3D::new(x, y, time)
            .mode(plotly::common::Mode::LinesMarkers) // Connect the points with lines
            .name(format!("Particle {}", id));
        plot.add_trace(trace);
    }

    plot.set_layout(
        plotly::Layout::new()
            .title("2D Particle Motion")
            .x_axis(Axis::new().title(Title::with_text("X Position (m)")))
            .y_axis(Axis::new().title(Title::with_text("Y Position (m)")))
            .z_axis(Axis::new().title(Title::with_text("Time (s)")))
            .width(1200)    // Set a default width
            .height(800),   // Set a default height
    );

    plot.write_html("output.html");
}