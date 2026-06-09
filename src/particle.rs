use std::collections::HashMap;

/// A single particle in a longitudinal wave, each connected to other particles
/// by linear springs.
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
    /// Create a new [`Particle`] with a mass of 1.0 kg, position of (0.0, 0.0,
    /// 0.0), velocity of <0.0, 0.0, 0.0>, acceleration of <0.0, 0.0, 0.0>, and
    /// no linked masses.
    ///
    /// # Return
    ///
    /// A new [`Particle`] with a mass of 1.0 kg, position of (0.0, 0.0,
    /// 0.0), velocity of <0.0, 0.0, 0.0>, acceleration of <0.0, 0.0, 0.0>, and
    /// no linked masses.
    pub fn new() -> Particle {
        Particle {
            mass: 1.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![0.0, 0.0, 0.0],
            acceleration: vec![0.0, 0.0, 0.0],
            linked_masses: HashMap::new(),
        }
    }

    /// Get a new [`ParticleBuilder`].
    ///
    /// # Return
    ///
    /// A new [`ParticleBuilder`].
    pub fn builder() -> ParticleBuilder {
        ParticleBuilder::new()
    }
}


/// The builder for the [`Particle`] class.
pub struct ParticleBuilder {}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {}
    }
}
