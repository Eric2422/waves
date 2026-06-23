//! Module to represent [`Particle`]s in a longitudinal wave.

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::vector3d::Vector3d;


/// Counter for the [`id`] property of the [`Particle`] class.
///
/// [`id`]: Particle::id
static PARTICLE_COUNTER: AtomicUsize = AtomicUsize::new(0);


/// A single particle in a longitudinal wave,
/// each connected to other particles by linear springs.
pub struct Particle {
    id: usize,
    /// The mass of this particle in kilograms (kg).
    pub mass: f64,
    /// The position of this particle as a 3D vector in meters (m).
    pub position: Vector3d,
    /// The velocity of this particle as a 3D vector in meters per second (m/s).
    pub velocity: Vector3d,
    /// The acceleration of this particle as a 3D vector
    /// in meters per second squared (m/s²).
    pub acceleration: Vector3d,
    /// The particles that this particle is linked to by springs
    /// mapped onto the respective spring constants in newtons per meters (N/m).
    linked_particles: HashMap<Particle, f64>,
}

impl Debug for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Particle")
            .field("id", &self.id)
            .field("mass", &self.mass)
            .field("position", &self.position)
            .field("velocity", &self.velocity)
            .field("acceleration", &self.acceleration)
            .field("linked_particles", &self.linked_particles)
            .finish()
    }
}

impl Clone for Particle {
    /// Create a deep copy of this [`Particle`] except for the [`id`] property,
    /// which still increments by 1, similarly to [`Particle::new()`].
    ///
    /// [`id`]: Particle::id
    fn clone(&self) -> Self {
        Self {
            id: PARTICLE_COUNTER.fetch_add(1, Ordering::SeqCst),
            mass: self.mass.clone(),
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone(),
            linked_particles: self.linked_particles.clone(),
        }
    }
}

impl Eq for Particle {}

impl Hash for Particle {
    /// Generate a hash based on based on [`Particle::id`].
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("{}, {:?}", self.mass, self.position)
    }
}

impl Particle {
    /// Create a new [`Particle`] based on the given [`ParticleBuilder`]
    /// instance.
    /// The [`id`] property will be assigned from the value stored in
    /// [`PARTICLE_COUNTER`],
    /// which increments by one (1) every time this function is called.
    /// Thus, no two [`Particle`]s will have an identical [`id`].
    ///
    /// [`id`]: Particle::id
    fn new(builder: ParticleBuilder) -> Particle {
        let new_particle = Particle {
            id: PARTICLE_COUNTER.fetch_add(1, Ordering::SeqCst),
            mass: builder.mass,
            position: builder.position,
            velocity: builder.velocity,
            acceleration: Vector3d(0.0, 0.0, 0.0),
            linked_particles: builder.linked_particles,
        };

        new_particle
    }

    /// Instantiates and returns a new [`ParticleBuilder`].
    pub fn builder() -> ParticleBuilder {
        ParticleBuilder::new()
    }
}


/// A builder for the [`Particle`] class,
/// allowing for a way to set the [`mass`], [`position`], [`velocity`],
/// [`acceleration`], and [`linked_particles`].
/// Note that since [`id`]s are predetermined in [`Particle::new()`],
/// the builder does not come with a method to set the [`id`].
///
/// [`mass`]: Particle::mass
/// [`position`]: Particle::position
/// [`velocity`]: Particle::velocity
/// [`acceleration`]: Particle::acceleration
/// [`linked_particles`]: Particle::linked_particles
/// [`id`]: Particle::id
pub struct ParticleBuilder {
    mass: f64,
    position: Vector3d,
    velocity: Vector3d,
    linked_particles: HashMap<Particle, f64>,
}

impl ParticleBuilder {
    /// Instantiate and return a new [`ParticleBuilder`] with a mass of 1.0 kg,
    /// position of (0.0, 0.0, 0.0) m, velocity of <0.0, 0.0, 0.0> m/s,
    /// acceleration of <0.0, 0.0, 0.0> m/s², and no linked [`Particle`]s.
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {
            mass: 1.0,
            position: Vector3d(0.0, 0.0, 0.0),
            velocity: Vector3d(0.0, 0.0, 0.0),
            linked_particles: HashMap::new(),
        }
    }

    /// Set the [`mass`] of the [`Particle`] in kilograms (kg).
    /// If the given new value for [`mass`] is non-positive,
    /// i.e., [`mass`] < 0.0 kg, the current [`mass`] remains unchanged.
    ///
    /// Can be chained with other setter methods.
    ///
    /// # Example
    ///
    /// ```rust
    /// let particle = ParticleBuilder::new()
    ///     .set_mass(2.0)
    ///     .set_position(1.0, 1.0, 1.0)
    ///     .set_velocity(0.5, 0.5, 0.5)
    ///     .build();
    /// ```
    ///
    /// [`mass`]: Particle::mass
    pub fn set_mass(mut self, mass: f64) -> ParticleBuilder {
        self.mass = mass;
        self
    }

    /// Set the [`position`] of the [`Particle`] as a 3D vector in meters (m).
    ///
    /// Can be chained with other setter methods.
    ///
    /// # Example
    ///
    /// ```rust
    /// let particle = ParticleBuilder::new()
    ///     .set_mass(2.0)
    ///     .set_position(1.0, 1.0, 1.0)
    ///     .set_velocity(0.5, 0.5, 0.5)
    ///     .build();
    /// ```
    ///
    /// [`position`]: Particle::position
    pub fn set_position(mut self, x: f64, y: f64, z: f64) -> ParticleBuilder {
        self.position = Vector3d(x, y, z);
        self
    }

    /// Set the [`velocity`] of the [`Particle`] as a 3D vector in meters per
    /// second (m/s).
    ///
    /// Can be chained with other setter methods.
    ///
    /// # Example
    ///
    /// ```rust
    /// let particle = ParticleBuilder::new()
    ///     .set_mass(2.0)
    ///     .set_position(1.0, 1.0, 1.0)
    ///     .set_velocity(0.5, 0.5, 0.5)
    ///     .build();
    /// ```
    ///
    /// [`velocity`]: Particle::velocity
    pub fn set_velocity(mut self, x: f64, y: f64, z: f64) -> ParticleBuilder {
        self.velocity = Vector3d(x, y, z);
        self
    }

    /// Link this [`Particle`] to another [`Particle`]
    /// with a spring of constant `spring_constant` in newtons per meter (N/m),
    /// updating [`linked_masses`] accordingly.
    ///
    /// If the given [`Particle`] already exists in [`linked_particles`],
    /// the pre-existing spring constant will be replaced with the new one.
    ///
    /// Can be chained with other setter methods.
    ///
    /// # Example
    ///
    /// ```rust
    /// let particle = ParticleBuilder::new()
    ///     .set_mass(2.0)
    ///     .set_position(1.0, 1.0, 1.0)
    ///     .set_velocity(0.5, 0.5, 0.5)
    ///     .build();
    /// ```
    ///
    /// [`linked_particles`]: Particle::linked_particles
    pub fn add_linked_particle(
        mut self,
        particle: Particle,
        spring_constant: f64,
    ) -> ParticleBuilder {
        self.linked_particles.insert(particle, spring_constant);
        self
    }

    /// Attempts to instantiate a new [`Particle`] object
    /// using the current values of [`mass`], [`position`], [`velocity`],
    /// [`acceleration`], and [`linked_particles`].
    ///
    /// [`mass`]: Particle::mass
    /// [`position`]: Particle::position
    /// [`velocity`]: Particle::velocity
    /// [`acceleration`]: Particle::acceleration
    /// [`linked_particles`]: Particle::linked_particles
    pub fn build(self) -> Particle {
        Particle::new(self)
    }
}
