use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Counter for the [`id`](Particle::id) property of the [`Particle`] class.
static PARTICLE_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A single particle in a longitudinal wave, each connected to other particles
/// by linear springs.
pub struct Particle {
    id: usize,
    /// The mass of this particle in kilograms (kg).
    mass: f64,
    /// The position of this particle in meters (m) as a vector in 3D space.
    position: Vec<f64>,
    /// The velocity of this particle in meters per second (m/s) as a vector in
    /// 3D space.
    velocity: Vec<f64>,
    /// The acceleration of this particle in meters per second squared (m/s²) as
    /// a vector in 3D space.
    acceleration: Vec<f64>,
    /// The particles that this particle is linked to by springs and the spring
    /// constant of the respective spring in newtons per meters (n/m).
    linked_particles: HashMap<Particle, f64>,
}

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("{}, {:?}", self.mass, self.position)
    }
}

impl PartialEq for Particle {
    /// Check if this [`Particle`] is considered equivalent to another
    /// [`Particle`], returning `true` if and only if they have the same [`id`].
    ///
    /// [`id`]: Particle::id
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Particle {}

impl Hash for Particle {
    /// Generate a hash based on based on [`Particle::id`].
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Particle {
    /// Create a new [`Particle`] with a mass of 1.0 kg, position of (0.0, 0.0,
    /// 0.0), velocity of <0.0, 0.0, 0.0> m/s, acceleration of <0.0, 0.0, 0.0>
    /// m/s², and no linked particles.
    ///
    /// The value stored in [`PARTICLE_COUNTER`] will increment by one (1) after
    /// calling.
    pub fn new() -> Particle {
        Particle {
            id: PARTICLE_COUNTER.fetch_add(1, Ordering::SeqCst),
            mass: 1.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![0.0, 0.0, 0.0],
            acceleration: vec![0.0, 0.0, 0.0],
            linked_particles: HashMap::new(),
        }
    }

    /// Instantiates and returns a new [`ParticleBuilder`].
    pub fn builder() -> ParticleBuilder {
        ParticleBuilder::new()
    }
}


/// A builder for the [`Particle`] class, allowing for a way to set the
/// [`mass`], [`position`], [`velocity`], [`acceleration`], and
/// [`linked_particles`]. Note that since [`id`]s are predetermined, the builder
/// does not come with a method to set the [`id`].
///
/// [`mass`]: Particle::mass
/// [`position`]: Particle::position
/// [`velocity`]: Particle::velocity
/// [`acceleration`]: Particle::acceleration
/// [`linked_particles`]: Particle::linked_particles
/// [`id`]: Particle::id
pub struct ParticleBuilder {
    mass: f64,
    position: Vec<f64>,
    velocity: Vec<f64>,
    acceleration: Vec<f64>,
    linked_particles: HashMap<Particle, f64>,
}

impl ParticleBuilder {
    /// Instantiate and return a new [`ParticleBuilder`] with a mass of 1.0 kg,
    /// position of (0.0, 0.0, 0.0) m, velocity of <0.0, 0.0, 0.0> m/s,
    /// acceleration of <0.0, 0.0, 0.0> m/s², and no linked [`Particle`]s.
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {
            mass: 1.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![0.0, 0.0, 0.0],
            acceleration: vec![0.0, 0.0, 0.0],
            linked_particles: HashMap::new(),
        }
    }

    /// Set the [`mass`] of the [`Particle`] in kilograms (kg).
    ///
    /// [`mass`]: Particle::mass
    pub fn set_mass(mut self, mass: f64) {
        self.mass = mass;
    }

    /// Set the [`position`] of the [`Particle`] as a 3D vector in meters (m).
    ///
    /// [`position`]: Particle::position
    pub fn set_position(mut self, x: f64, y: f64, z: f64) {
        self.position = vec![x, y, z];
    }

    /// Set the [`velocity`] of the [`Particle`] as a 3D vector in meters per
    /// second (m/s).
    ///
    /// [`velocity`]: Particle::velocity
    pub fn set_velocity(mut self, x: f64, y: f64, z: f64) {
        self.velocity = vec![x, y, z];
    }

    /// Set the [`acceleration`] of the [`Particle`] as a 3D vector in meters
    /// per second squared (m/s²).
    ///
    /// [`acceleration`]: Particle::acceleration
    pub fn set_acceleration(mut self, x: f64, y: f64, z: f64) {
        self.acceleration = vec![x, y, z];
    }

    /// Link this [`Particle`] to another [`Particle`] with a spring of constant
    /// `spring_constant` in newtons per meter (N/m), updating
    /// [`linked_masses`] accordingly.
    ///
    /// If the given [`Particle`] already exists in [`linked_particles`], the
    /// pre-existing spring constant will be replaced with the new one.
    ///
    /// [`linked_particles`]: Particle::linked_particles
    pub fn add_linked_particle(mut self, particle: Particle, spring_constant: f64) {
        self.linked_particles.insert(particle, spring_constant);
    }
}
