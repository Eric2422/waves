//! Module to represent [`Particle`]s in a longitudinal wave.

use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use uom::si::{
    f64::{Length, Mass},
    mass::kilogram,
};

use crate::{vector3d, vector3d::Vector3d};

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
    /// The springs attached to this [`Particle`].
    attached_springs: HashSet<Spring>,
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
            attached_springs: self.attached_springs.clone(),
        }
    }
}

impl Debug for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Particle")
            .field("id", &self.id)
            .field("mass", &self.mass)
            .field("position", &self.position)
            .field("velocity", &self.velocity)
            .field("acceleration", &self.acceleration)
            .field("attached_springs", &self.attached_springs)
            .finish()
    }
}

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Particle {}: m = {:?} kg, r = {} m, v = {} m/s, a = {} m/s²",
            self.id, self.mass, self.position, self.velocity, self.acceleration
        )
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

impl Particle {
    /// Instantiates and returns a new default [`ParticleBuilder`].
    pub fn builder() -> ParticleBuilder {
        ParticleBuilder::default()
    }
}

/// A builder for the [`Particle`] class,
/// allowing for a way to set the [`mass`], [`position`], [`velocity`],
/// [`acceleration`], and [`attached_springs`].
///
/// Note that since [`id`]s are predetermined in [`ParticleBuilder::build()`],
/// the builder does not come with a method to set the [`id`].
///
/// [`mass`]: Particle::mass
/// [`position`]: Particle::position
/// [`velocity`]: Particle::velocity
/// [`acceleration`]: Particle::acceleration
/// [`attached_springs`]: Particle::attached_springs
/// [`id`]: Particle::id
#[derive(Default)]
pub struct ParticleBuilder {
    mass: f64,
    position: Vector3d,
    velocity: Vector3d,
    attached_springs: HashSet<Spring>,
}

impl ParticleBuilder {
    /// Instantiate and return a new [`ParticleBuilder`] with a mass of 1.0 kg,
    /// position of (0.0, 0.0, 0.0) m, velocity of <0.0, 0.0, 0.0> m/s,
    /// acceleration of <0.0, 0.0, 0.0> m/s², and no attached [`Spring`]s.
    pub fn new_1kg() -> ParticleBuilder {
        ParticleBuilder::new(1.0)
    }

    /// Instantiate and return a new [`ParticleBuilder`] with a given mass,
    /// position of (0.0, 0.0, 0.0) m, velocity of <0.0, 0.0, 0.0> m/s,
    /// acceleration of <0.0, 0.0, 0.0> m/s², and no attached [`Spring`]s.
    pub fn new(mass: f64) -> ParticleBuilder {
        ParticleBuilder {
            mass,
            position: Vector3d::zero(),
            velocity: Vector3d::zero(),
            attached_springs: HashSet::new(),
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
        if mass > 0.0 {
            self.mass = mass;
        };
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
        self.position = vector3d!(x, y, z);
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
        self.velocity = vector3d!(x, y, z);
        self
    }

    /// Link this [`Particle`] to another [`Particle`]
    /// with a spring of constant `spring_constant` in newtons per meter (N/m),
    /// updating [`attached_springs`] accordingly.
    ///
    /// If the given [`Particle`] already exists in [`attached_springs`],
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
    /// [`attached_springs`]: Particle::attached_springs
    pub fn attach_spring(mut self, spring: Spring, spring_constant: f64) -> ParticleBuilder {
        self.attached_springs.insert(spring);
        self
    }

    /// Attempts to instantiate a new [`Particle`] object
    /// using the current values of [`mass`], [`position`], [`velocity`],
    /// [`acceleration`], and [`attached_springs`].
    ///
    /// The [`id`] property will be assigned from the value stored in
    /// [`PARTICLE_COUNTER`],
    /// which increments by one (1) every time this function is called.
    /// Thus, no two [`Particle`]s will have an identical [`id`].
    ///
    ///
    /// [`mass`]: Particle::mass
    /// [`position`]: Particle::position
    /// [`velocity`]: Particle::velocity
    /// [`acceleration`]: Particle::acceleration
    /// [`attached_springs`]: Particle::attached_springs
    pub fn build(self) -> Particle {
        Particle {
            id: PARTICLE_COUNTER.fetch_add(1, Ordering::SeqCst),
            mass: self.mass,
            position: self.position,
            velocity: self.velocity,
            acceleration: Vector3d::zero(),
            attached_springs: self.attached_springs,
        }
    }
}


/// A spring of a given stiffness connecting two [`Particle`]s.
#[derive(Clone, Debug)]
struct Spring {
    particles: [Particle; 2],
    spring_constant: f64,
    resting_length: f64,
}

impl Hash for Spring {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.particles.hash(state);
    }
}

impl PartialEq for Spring {
    fn eq(&self, other: &Self) -> bool {
        self.particles == other.particles
    }
}

impl Eq for Spring {}
