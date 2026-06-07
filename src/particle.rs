use std::collections::HashMap;

/// A single particle in a longitudinal wave, each connected to other particles by linear springs.
struct Particle {
    mass: f64,
    position: Vec<f64>,
    velocity: Vec<f64>,
    acceleration: Vec<f64>,
    linked_masses: HashMap<Particle, f64>,
}

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("{}, {:?}", self.mass, self.position)
    }
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            mass: 1.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![0.0, 0.0, 0.0],
            acceleration: vec![0.0, 0.0, 0.0],
            linked_masses: HashMap::new(),
        }
    }

    pub fn builder() -> ParticleBuilder {
        ParticleBuilder::new()
    }
}

pub struct ParticleBuilder {}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {}
    }
}
