use crate::structs::Particle;
use crate::util::*;

pub(crate) fn resolve_gravitation_force(g_constant: f64, particle_i: &Particle, particle_j: &Particle) -> [f64; 2] {
    let  distance = distance(particle_i.position, particle_j.position);

    let angle = points_to_horizontal_angle(particle_j.position, particle_i.position);
    let g_force = (g_constant * particle_i.mass * particle_j.mass) / (distance.powi(2));
    vector_to_components(g_force, angle)
}

pub(crate) fn resolve_collision(p1: &Particle, p2: &Particle, restitution: f64, delta_t: f64) -> [f64; 4] {
    let v1 = [
        (p1.position[0] - p1.previous_position[0]) / delta_t,
        (p1.position[1] - p1.previous_position[1]) / delta_t,
    ];
    let v2 = [
        (p2.position[0] - p2.previous_position[0]) / delta_t,
        (p2.position[1] - p2.previous_position[1]) / delta_t,
    ];

    let mut normal = [
        p2.position[0] - p1.position[0],
        p2.position[1] - p1.position[1],
    ];
    let distance = (normal[0].powi(2) + normal[1].powi(2)).sqrt();

    if distance == 0.0 {
        return [0.0, 0.0, 0.0, 0.0];
    }

    normal[0] /= distance;
    normal[1] /= distance;

    let relative_velocity = [
        v2[0] - v1[0],
        v2[1] - v1[1],
    ];

    let vel_along_normal = relative_velocity[0] * normal[0] + relative_velocity[1] * normal[1];

    if vel_along_normal > 0.0 {
        return [0.0, 0.0, 0.0, 0.0];
    }

    // Compute impulse scalar
    let impulse_scalar = -(1.0 + restitution) * vel_along_normal
        / (1.0 / p1.mass + 1.0 / p2.mass);

    // Impulse vector
    let impulse = [impulse_scalar * normal[0], impulse_scalar * normal[1]];

    // Compute new velocity adjustments
    let v1_adjustment = [
        -impulse[0] / p1.mass, // Negative because impulse opposes velocity
        -impulse[1] / p1.mass,
    ];
    let v2_adjustment = [
        impulse[0] / p2.mass, // Positive because impulse adds velocity
        impulse[1] / p2.mass,
    ];

    // Convert adjustments into position changes for Verlet integration
    [
        v1_adjustment[0] * delta_t,
        v1_adjustment[1] * delta_t,
        v2_adjustment[0] * delta_t,
        v2_adjustment[1] * delta_t,
    ]
}