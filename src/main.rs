mod models;
use models::*;
mod util;
use util::*;
mod structs;
use structs::*;
mod interactions;
use interactions::*;

use macroquad::prelude::*;
use std::convert::Into;
use ::rand::Rng;
use ::rand::thread_rng;
fn window_conf() -> Conf {
    Conf {
        window_title: "Physics Simulation".to_owned(),
        window_width: 800,  // Adjust the window width
        window_height: 800, // Adjust the window height
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut selected_model: Model = blank_system();
    let mut scale_factor = selected_model.scale_factor;
    let mut scale_ref = scale_factor;
    let mut g_constant = selected_model.g_constant;

    let mut elapsed_time = 0.0;
    let mut paused = true;
    let mut change_simulation = false;

    let delta = 1.0;

    let mut custom_velocity = [0.0, 0.0];
    let mut change_velocity = false;

    let mut custom_mass = 1.0;
    let mut change_mass = false;

    loop {
        clear_background(BLACK);

        // Zoom and Speed
        if is_key_pressed(KeyCode::RightBracket) {
            if !is_key_down(KeyCode::LeftShift) {
                scale_factor = scale_factor * (11f32 / 10f32).powi(1);
            }else{
                selected_model.change_speed(selected_model.delta_t * (21f64 / 20f64).powi(1));
            }
        }
        if is_key_pressed(KeyCode::LeftBracket) {
            if !is_key_down(KeyCode::LeftShift) {
                scale_factor = scale_factor * (11f32 / 10f32).powi(-1);
            } else{
                selected_model.change_speed(selected_model.delta_t * (21f64 / 20f64).powi(-1));
            }
        }

        // Calculate scaled mouse position
        let mut world_mouse_pos = screen_to_world(mouse_position(),scale_factor);
        world_mouse_pos[0] = world_mouse_pos[0].round();
        world_mouse_pos[1] = world_mouse_pos[1].round();

        if is_key_pressed(KeyCode::Up) {
            if !is_key_down(KeyCode::LeftShift){
                if change_velocity {
                    custom_velocity[1] += delta;
                }
                if change_mass {
                    custom_mass += delta;
                    if custom_mass == 0.0 {
                        custom_mass = 0.1;
                    }
                }
            }else{
                if change_velocity {
                    custom_velocity[1] += delta / 10.0;
                }
                if change_mass {
                    custom_mass += delta / 10.0;
                    if custom_mass == 0.0 {
                        custom_mass = 0.1;
                    }
                }
            }
        }
        if is_key_pressed(KeyCode::Down) {
            if !is_key_down(KeyCode::LeftShift){
                if change_velocity {
                    custom_velocity[1] -= delta;
                }
                if change_mass {
                    custom_mass -= delta;
                    if custom_mass == 0.0 {
                        custom_mass = -0.1;
                    }
                }
            }else{
                if change_velocity {
                    custom_velocity[1] -= delta / 10.0;
                }
                if change_mass {
                    custom_mass -= delta / 10.0;
                    if custom_mass == 0.0 {
                        custom_mass = -0.1;
                    }
                }
            }
        }
        if is_key_pressed(KeyCode::Left) {
            if !is_key_down(KeyCode::LeftShift){
                if change_velocity {
                    custom_velocity[0] -= 1.0;
                }
            }else{
                if change_velocity {
                    custom_velocity[0] -= 0.1;
                }
            }
        }
        if is_key_pressed(KeyCode::Right) {
            if !is_key_down(KeyCode::LeftShift){
                if change_velocity {
                    custom_velocity[0] += 1.0;
                }
            }else{
                if change_velocity {
                    custom_velocity[0] += 0.1;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            selected_model.particles.push(
                Particle::new(
                    world_mouse_pos,
                    custom_velocity,
                    custom_mass,
                    selected_model.delta_t,
                    "/".into()
                )
            );
        } // Custom Particle
        if is_mouse_button_pressed(MouseButton::Left) {
            let mut rng = thread_rng();
            let mut random_mass: f64 = rng.gen_range(0.0..10.0);

            random_mass = random_mass.trunc();
            if random_mass == 0.0 { random_mass = 1.0; }

            selected_model.particles.push(
                Particle::new(
                    world_mouse_pos,
                    [0.0, 0.0],
                    random_mass,
                    selected_model.delta_t,
                    "/".into()
                )
            );
        } // Random Particle

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        } // Toggle Pause

        if is_key_pressed(KeyCode::Key1){ // Select model 1
            if !is_key_down(KeyCode::LeftShift) {
                if is_key_down(KeyCode::LeftControl) {
                    selected_model = blank_system();
                    change_simulation = true;
                }
            }else{
                change_mass = !change_mass;
                change_velocity = false;
            }
        } // Shift+1 -> Select Mass | Left Ctrl+1 -> Select Model 1
        if is_key_pressed(KeyCode::Key2) { // Select model 2
            if !is_key_down(KeyCode::LeftShift) {
                if is_key_down(KeyCode::LeftControl) {
                    selected_model = solar_system();
                    change_simulation = true;
                }
            }else{
                change_velocity = !change_velocity;
                change_mass = false;
            }
        } // Shift+1 -> Select Velocity | Left Ctrl+2 -> Select Model 2
        if is_key_pressed(KeyCode::Key3) { // Select model 3
            if is_key_down(KeyCode::LeftControl) {
                selected_model = earth_moon_system();
                change_simulation = true;
            }
            change_velocity = false;
            change_mass = false;
        } // Left Ctrl+2 -> Select Model 3

        if change_simulation {
            g_constant = selected_model.g_constant;
            scale_factor = selected_model.scale_factor;
            selected_model.particles = selected_model.particles.clone();
            scale_ref = scale_factor;
            elapsed_time = 0.0;
            paused = true;
            change_simulation = false;
        } // Update simulation parameters

        if is_key_pressed(KeyCode::R) {
            selected_model.reset();
            elapsed_time = 0.0;
            paused = true;
        } // Reset simulation

        if !paused {
            // Phase 1: Resolve overlaps
            for i in 0..selected_model.particles.len() {
                for j in (i + 1)..selected_model.particles.len() {
                    let distance = distance(
                        selected_model.particles[i].position,
                        selected_model.particles[j].position,
                    );

                    if distance < selected_model.particles[i].radius + selected_model.particles[j].radius {
                        let overlap = selected_model.particles[i].radius
                            + selected_model.particles[j].radius
                            - distance;

                        if overlap > 0.0 {
                            let total_mass = selected_model.particles[i].mass + selected_model.particles[j].mass;
                            let mass_ratio_i = selected_model.particles[j].mass / total_mass;
                            let mass_ratio_j = selected_model.particles[i].mass / total_mass;

                            // Calculate separation vector
                            let mut separation_vector = [
                                selected_model.particles[j].position[0] - selected_model.particles[i].position[0],
                                selected_model.particles[j].position[1] - selected_model.particles[i].position[1],
                            ];

                            let separation_magnitude = (separation_vector[0].powi(2) + separation_vector[1].powi(2)).sqrt();
                            separation_vector[0] /= separation_magnitude;
                            separation_vector[1] /= separation_magnitude;

                            // Apply separation proportional to mass
                            selected_model.particles[i].position[0] -= separation_vector[0] * overlap * mass_ratio_i;
                            selected_model.particles[i].position[1] -= separation_vector[1] * overlap * mass_ratio_i;
                            selected_model.particles[j].position[0] += separation_vector[0] * overlap * mass_ratio_j;
                            selected_model.particles[j].position[1] += separation_vector[1] * overlap * mass_ratio_j;
                        }

                    }
                }
            }

            // Phase 2: Apply gravitational forces and resolve collisions
            for i in 0..selected_model.particles.len() {
                selected_model.particles[i].force = [0.0, 0.0];

                for j in 0..selected_model.particles.len() {
                    if i != j {
                        let distance = distance(
                            selected_model.particles[i].position,
                            selected_model.particles[j].position,
                        );

                        let mut g_components = resolve_gravitation_force(
                            g_constant,
                            &selected_model.particles[i],
                            &selected_model.particles[j],
                        );

                        // Prevent gravitational forces for overlapping particles
                        if distance <= selected_model.particles[i].radius + selected_model.particles[j].radius {
                            g_components = [0.0, 0.0];
                        }

                        selected_model.particles[i].force[0] += g_components[0];
                        selected_model.particles[i].force[1] += g_components[1];

                        // Handle collisions
                        if check_collision(
                            &selected_model.particles[i],
                            &selected_model.particles[j],
                        ) {
                            let impulse = resolve_collision(
                                &selected_model.particles[i],
                                &selected_model.particles[j],
                                selected_model.restitution,
                                selected_model.delta_t,
                            );
                            selected_model.particles[i].previous_position[0] -= impulse[0];
                            selected_model.particles[i].previous_position[1] -= impulse[1];
                            selected_model.particles[j].previous_position[0] -= impulse[2];
                            selected_model.particles[j].previous_position[1] -= impulse[3];
                        }
                    }
                }
            }
        }

        // Update particles
        if !paused {
            for i in 0..selected_model.particles.len() {
                selected_model.particles[i].update(selected_model.delta_t);
            }
            elapsed_time += selected_model.delta_t;
        }


        draw_text(
            &format!("Selected Model: {:#}", selected_model.name),
            20.0,
            20.0,
            16.0,
            YELLOW,
        );

        if paused {
            draw_text(&format!("Frames elapsed: {}", elapsed_time.trunc()), 20.0, 50.0, 16.0, RED);
        } else {
            draw_text(&format!("Frames elapsed: {}", elapsed_time.trunc()), 20.0, 50.0, 16.0, YELLOW);
        }
        draw_text(
            &format!("Number of particles: {}", selected_model.particles.len()),
            20.0,
            80.0,
            16.0,
            YELLOW,
        );

        if is_key_down(KeyCode::LeftShift) {
            draw_text(&format!("Delta: {:?}", round_to_place(delta / 10.0,2)), 20.0, 110.0, 16.0, RED);
        }else{
            draw_text(&format!("Delta: {:?}", round_to_place(delta,2)), 20.0, 110.0, 16.0, RED, );
        }

        if !change_mass {
            draw_text(&format!("Mass: {:?}", round_to_place(custom_mass,2)), 20.0, 140.0, 16.0, YELLOW, );
        } else {
            draw_text(&format!("Mass: {:?}", round_to_place(custom_mass,2)), 20.0, 140.0, 16.0, RED, );
        }

        if !change_velocity {
            draw_text(&format!("Velocity: {:?}", [round_to_place(custom_velocity[0],2),round_to_place(custom_velocity[1],2)]), 20.0, 170.0, 16.0, YELLOW, );
        } else {
            draw_text(&format!("Velocity: {:?}", [round_to_place(custom_velocity[0],2),round_to_place(custom_velocity[1],2)]), 20.0, 170.0, 16.0, RED, );
        }

        draw_text(&format!("Restitution Value: {:.2}", selected_model.restitution), 20.0, screen_height() - 210.0, 16.0, RED);
        draw_text(&format!("G Constant: {}", selected_model.g_constant), 20.0, screen_height() - 190.0, 16.0, RED);

        draw_text(&format!("Zoom: {}%", round_to_place((scale_factor/ scale_ref * 100.0).into(),2)), 20.0, screen_height() - 160.0, 16.0, RED);
        draw_text(&format!("Time Step: {:.3}", selected_model.delta_t), 20.0, screen_height() - 130.0, 16.0, RED);
        draw_text(&format!("({}% of default: {:.3})",
                           round_to_place((selected_model.delta_t/selected_model.default_delta_t)*100.0,2),
                           selected_model.default_delta_t),
                  20.0,
                  screen_height() - 100.0,
                  16.0,
                  RED
        );

        draw_text(&format!("Mouse X: {}", world_mouse_pos[0]), 20.0, screen_height() - 70.0, 16.0, RED);
        draw_text(&format!("Mouse Y: {}", world_mouse_pos[1]), 20.0, screen_height() - 40.0, 16.0, RED);


        selected_model.draw(scale_factor);
        next_frame().await;
    }
}