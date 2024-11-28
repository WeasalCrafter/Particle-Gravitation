use macroquad::color::{Color, WHITE};
use macroquad::prelude::{draw_circle, draw_text, screen_height, screen_width};

#[derive(Clone)]
pub(crate) struct Particle {
    pub(crate) position: [f64; 2],            // meters
    pub(crate) previous_position: [f64; 2],   // meters
    pub(crate) radius: f64,                   // meters
    pub(crate) acceleration: [f64; 2],        // m/s^2
    pub(crate) force: [f64; 2],               // newtons
    pub(crate) mass: f64,                     // kilograms
    pub(crate) name: String,
}

pub(crate) struct Model {
    pub(crate) particles: Vec<Particle>,            // predefined particles
    pub(crate) default_particles: Vec<Particle>,    // default particles; auto-set
    pub(crate) delta_t: f64,                        // time step
    pub(crate) default_delta_t: f64,                // default time step; auto set
    pub(crate) scale_factor: f32,                   // scale factor
    pub(crate) g_constant: f64,                     // G constant
    pub(crate) do_collisions: bool,                 // should collisions be calculated ?
    pub(crate) restitution: f64,                    // restitution value for collisions
    pub(crate) name: String,                        // Model name
}

impl Particle {
    pub(crate) fn new(initial_position: [f64; 2], initial_velocity: [f64; 2], mass: f64, radius: f64, time_step: f64, name: String) -> Self {
        let mut previous_position = [initial_position[0], initial_position[1]];
        previous_position[0] -= initial_velocity[0] * time_step;
        previous_position[1] -= initial_velocity[1] * time_step;

        Particle {
            position: initial_position,
            previous_position,
            radius,
            acceleration: [0.0, 0.0],
            force: [0.0, 0.0],
            mass,
            name,
        }
    }

    pub(crate) fn update(&mut self, delta_t: f64) {
        self.acceleration[0] = self.force[0] / self.mass;
        self.acceleration[1] = self.force[1] / self.mass;

        let mut new_position = [0.0, 0.0];
        new_position[0] = 2.0 * self.position[0] - self.previous_position[0] + self.acceleration[0] * delta_t.powi(2);
        new_position[1] = 2.0 * self.position[1] - self.previous_position[1] + self.acceleration[1] * delta_t.powi(2);

        self.previous_position = self.position;
        self.position = new_position;
    }
}

impl Model{
    pub(crate) fn new(defaults: Vec<Particle>, delta_t: f64, scale_factor: f32, g_constant: f64, do_collisions: bool, restitution: f64, name: String) -> Model {
        Model {
            particles: defaults.clone(),
            default_particles: defaults,
            delta_t: delta_t.clone(),
            default_delta_t: delta_t,
            scale_factor,
            g_constant,
            do_collisions,
            restitution,
            name
        }
    }

    pub(crate) fn reset(&mut self) {
        self.particles = self.default_particles.clone();
        self.delta_t = self.default_delta_t.clone();
    }

    pub(crate) fn change_speed(&mut self, delta_t: f64) {
        let current_particles = &self.particles;
        let mut adjusted_particles: Vec<Particle> = Vec::new();

        for i in 0..current_particles.len() {
            let current_velocity = [
                (current_particles[i].position[0] - current_particles[i].previous_position[0])/self.delta_t,
                (current_particles[i].position[1] - current_particles[i].previous_position[1])/self.delta_t,
            ];

            let new_particle = Particle::new(
                current_particles[i].position,
                current_velocity,
                current_particles[i].mass,
                current_particles[i].radius,
                delta_t,
                current_particles[i].name.clone(),
            );
            adjusted_particles.push(new_particle);
        }
        self.particles = adjusted_particles;
        self.delta_t = delta_t;
    }

    pub(crate) fn draw(&mut self, scale_factor: f32){
        for i in 0..self.particles.len() {
            let screen_x = self.particles[i].position[0] as f32 * scale_factor + screen_width() / 2.0; // Center the screen
            let screen_y = self.particles[i].position[1] as f32 * scale_factor + screen_height() / 2.0;
            let color: Color = Color::new(0.00, 0.89, 0.19, 1.00);

            if self.do_collisions {
                draw_circle(screen_x, screen_y, (self.particles[i].radius * scale_factor as f64) as f32, WHITE); // Draw particle
            }else{
                draw_circle(screen_x, screen_y, self.particles[i].radius as f32, WHITE); // Draw particle
            }
            if self.particles[i].name != "/".to_string() {
                draw_text(
                    &format!("{}", self.particles[i].name),
                    screen_x + 10.0,
                    screen_y + 10.0,
                    16.0,
                    color,
                );
            }
        }
    }
}