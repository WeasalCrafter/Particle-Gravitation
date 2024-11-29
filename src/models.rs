use crate::structs::*;
static G_CONSTANT: f64 = 6.674e-11;

pub(crate) fn solar_system() -> Model {
    let restitution = 1.0;
    let delta_t = 1.0e5;
    let scale_factor = 1e-9;
    let g_constant = G_CONSTANT;
    let do_collisions = false;
    let particles = vec![
        Particle::new( // Sun
                       [0.0, 0.0],
                       [0.0, 0.0],
                       1.989e30, // Mass of the Sun
                       delta_t,
                       String::from("Sun"),
        ),
        Particle::new( // Mercury
                       [4.6e10, 0.0], // Perihelion distance in meters
                       [0.0, 53703.3518507], // Orbital velocity in m/s
                       3.285e23, // Mass of Mercury
                       delta_t,
                       String::from("Mercury"),
        ),
        Particle::new( // Venus
                       [1.0875e11, 0.0], // Average distance from Sun in meters
                       [0.0, 34927.3531777], // Orbital velocity in m/s
                       4.867e24, // Mass of Venus
                       delta_t,
                       String::from("Venus"),
        ),
        Particle::new( // Earth
                       [1.4765e11, 0.0], // Average distance from Sun in meters
                       [0.0, 29975.3030751], // Orbital velocity in m/s
                       5.972e24, // Mass of Earth
                       delta_t,
                       String::from("Earth"),
        ),
        Particle::new( // Mars
                       [2.279e11, 0.0], // Average distance from Sun in meters
                       [0.0, 24117.9259962], // Orbital velocity in m/s
                       6.417e23, // Mass of Mars
                       delta_t,
                       String::from("Mars"),
        ),
        Particle::new( // Jupiter
                       [7.785e11, 0.0], // Average distance from Sun in meters
                       [0.0, 13069.708962], // Orbital velocity in m/s
                       1.898e27, // Mass of Jupiter
                       delta_t,
                       String::from("Jupiter"),
        ),
        Particle::new( // Saturn
                       [1.4335e12, 0.0], // Average distance from Sun in meters
                       [0.0, 9690.4862238], // Orbital velocity in m/s
                       5.683e26, // Mass of Saturn
                       delta_t,
                       String::from("Saturn"),
        ),
        Particle::new( // Uranus
                       [2.8725e12, 0.0], // Average distance from Sun in meters
                       [0.0, 6835.08288589], // Orbital velocity in m/s
                       8.681e25, // Mass of Uranus
                       delta_t,
                       String::from("Uranus"),
        ),
        Particle::new( // Neptune
                       [4.4951e12, 0.0], // Average distance from Sun in meters
                       [0.0, 5477.9200121], // Orbital velocity in m/s
                       1.024e26, // Mass of Neptune
                       delta_t,
                       String::from("Neptune"),
        ),
        Particle::new( // Pluto (Dwarf Planet)
                       [5.9064e12, 0.0], // Average distance from Sun in meters
                       [0.0, 4748.04182444], // Orbital velocity in m/s
                       1.309e22, // Mass of Pluto
                       delta_t,
                       String::from("Pluto"),
        ),


    ];
    Model::new(
        particles,
        delta_t,
        scale_factor,
        g_constant,
        do_collisions,
        restitution,
        "Solar System".into()
    )
}
pub(crate) fn earth_moon_system() -> Model {
    let restitution = 1.0;
    let delta_t = 1.0e3;
    let scale_factor = 6.054754e-7;
    let g_constant = G_CONSTANT;
    let do_collisions = false;
    let particles = vec![
        Particle::new( // Earth
                       [0.0, 0.0], // Average distance from Sun in meters
                       [0.0, 0.0], // Orbital velocity in m/s
                       5.972e24, // Mass of Earth
                       delta_t,
                       String::from("Earth"),
        ),
        Particle::new( // Moon
                       [3.844e8, 0.0], // Average distance from Sun in meters
                       [0.0, 1018.26616017], // Orbital velocity in m/s
                       7.34767309e22, // Mass of Moon
                       delta_t,
                       String::from("Moon"),
        ),
    ];
    Model::new(
        particles,
        delta_t,
        scale_factor,
        g_constant,
        do_collisions,
        restitution,
        "Earth-Moon System".into()
    )
}
pub(crate) fn blank_system() -> Model {
    let restitution = 0.75;
    let delta_t = 1.0;
    let scale_factor = 1.0;
    let g_constant = 0.01;
    let do_collisions = true;
    let particles = vec![];
    Model::new(
        particles,
        delta_t,
        scale_factor,
        g_constant,
        do_collisions,
        restitution,
        "Custom".into()
    )
}
