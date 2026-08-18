//! Module to represent [`Particle`]s in a wave.

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use uom::{
    ConstZero,
    fmt::DisplayStyle::Abbreviation,
    si::{
        f64::{Length, Mass, Velocity},
        length::meter,
        mass::kilogram,
        velocity::meter_per_second,
    },
};

use crate::{
    dimension::{self, SpringStiffness},
    vector3d,
    vectors::Vector3d,
};

/// Counter for the [`id`] property of the [`Particle`] class.
///
/// [`id`]: Particle::id
static PARTICLE_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A single particle in a longitudinal wave,
/// each connected to other particles by linear springs.
pub struct Particle<'a> {
    /// A unique unsigned integer identifying this [`Particle`].
    pub id: usize,
    /// The mass of this [`Particle`]  in kilograms (kg).
    pub mass: Mass,
    /// The position of this [`Particle`] as a 3D vector in metres (m).
    pub position: Vector3d,
    /// The velocity of this [`Particle`] as a 3D vector in metres per second
    /// (m/s).
    pub velocity: Vector3d,
    /// The acceleration of this [`Particle`] as a 3D vector
    /// in metres per second squared (m/s²).
    pub acceleration: Vector3d,
    /// The [`Spring`]s attached to this [`Particle`].
    attached_springs: HashMap<&'a Particle<'a>, Rc<Spring>>,
}

impl<'a> Clone for Particle<'a> {
    /// Create a deep copy of this [`Particle`] except for the [`id`] property,
    /// which still increments by 1, similarly to [`ParticleBuilder::new()`].
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

impl<'a> Debug for Particle<'a> {
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

impl<'a> Display for Particle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Particle {}: m = {}; r = {} m; v = {} m/s; a = {} m/s²",
            self.id,
            self.mass.into_format_args(kilogram, Abbreviation),
            self.position,
            self.velocity,
            self.acceleration
        )
    }
}

impl<'a> Eq for Particle<'a> {}

impl<'a> Hash for Particle<'a> {
    /// Generate a hash based on based on [`Particle::id`].
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<'a> PartialEq for Particle<'a> {
    /// Check if this [`Particle`] is considered equivalent to another
    /// [`Particle`], returning `true` if and only if they have the same [`id`].
    ///
    /// [`id`]: Particle::id
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Particle<'a> {
    /// Instantiates and returns a new default [`ParticleBuilder`]
    /// (see [`ParticleBuilder::new()`]).
    pub fn builder() -> ParticleBuilder {
        ParticleBuilder::default()
    }

    /// Links this [`Particle`] to another [`Particle`] with a new [`Spring`]
    /// of a given spring stiffness, updating [`attached_springs`] accordingly.
    ///
    /// If the given [`Particle`] already exists in [`attached_springs`],
    /// the pre-existing spring constant will be replaced with the new one.
    ///
    /// [`attached_springs`]: Particle::attached_springs
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uom::si::f64
    ///
    /// let particle = ParticleBuilder::new()
    /// ```
    pub fn attach_spring(mut self, particle: &'a Particle, spring_stiffness: SpringStiffness) {
        let new_spring = Rc::new(Spring::new(
            spring_stiffness,
            Length::new::<meter>((particle.position - self.position).get_magnitude()),
        ));
        self.attached_springs.insert(particle, new_spring);
    }
}

/// A builder for the [`Particle`] class,
/// allowing for a way to Sets the [`mass`], [`position`], [`velocity`],
/// [`acceleration`], and [`attached_springs`].
///
/// Note that since [`id`]s are predetermined in [`ParticleBuilder::build()`],
/// the builder does not come with a method to Sets the [`id`].
///
/// [`mass`]: Particle::mass
/// [`position`]: Particle::position
/// [`velocity`]: Particle::velocity
/// [`acceleration`]: Particle::acceleration
/// [`attached_springs`]: Particle::attached_springs
/// [`id`]: Particle::id
#[derive(Default)]
pub struct ParticleBuilder {
    /// Field used to set [`Particle::mass`].
    mass: Mass,
    /// Field used to set [`Particle::position`].
    position: Vector3d,
    /// Field used to set [`Particle::velocity`].
    velocity: Vector3d,
}

impl<'a> ParticleBuilder {
    /// Instantiates and returns a new [`ParticleBuilder`] with a given
    /// [`mass`], [`position`] of (0.0, 0.0, 0.0) m, [`velocity`] of <0.0,
    /// 0.0, 0.0> m/s, and no attached [`Spring`]s.
    ///
    /// [`mass`]: Particle::mass
    /// [`position`]: ParticleBuilder::position
    /// [`velocity`]: ParticleBuilder::velocity
    pub fn new(mass: Mass) -> ParticleBuilder {
        ParticleBuilder {
            mass,
            position: Vector3d::zero(),
            velocity: Vector3d::zero(),
        }
    }

    /// Instantiates and returns a [`ParticleBuilder`] with an infinite
    /// [`mass`], [`position`] of (0.0, 0.0, 0.0) m, [`velocity`] of <0.0,
    /// 0.0, 0.0> m/s, and no attached [`Spring`]s.
    ///
    /// An infinite [`mass`] means that the [`Particle`] is effectively fixed in
    /// place.
    ///
    /// [`mass`]: Particle::mass
    /// [`position`]: ParticleBuilder::position
    /// [`velocity`]: ParticleBuilder::velocity
    pub fn new_fixed() -> ParticleBuilder {
        ParticleBuilder::new(Mass::new::<kilogram>(f64::INFINITY))
    }

    /// Sets the [`mass`] of the [`Particle`] in kilograms (kg).
    /// If the given new value for [`mass`] is non-positive,
    /// i.e., [`mass`] < 0.0 kg, the current value remains unchanged.
    ///
    /// Can be chained with other setter methods.
    ///
    /// [`mass`]: Particle::mass
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uom::si::f64
    ///
    /// let particle = ParticleBuilder::new()
    ///     .set_mass(uom::si::f64::Mass::new::<kilogram>(2.0))
    ///     .set_position(
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///     )
    ///     .set_velocity(
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///     )
    ///     .build();
    /// ```
    pub fn set_mass(mut self, mass: Mass) -> ParticleBuilder {
        if mass > Mass::ZERO {
            self.mass = mass;
        };
        self
    }

    /// Sets the [`position`] of the [`Particle`] as a 3D vector in metres (m).
    ///
    /// Can be chained with other setter methods.
    ///
    /// [`position`]: Particle::position
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uom::si::f64
    ///
    /// let particle = ParticleBuilder::new()
    ///     .set_mass(uom::si::f64::Mass::new::<kilogram>(2.0))
    ///     .set_position(
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///     )
    ///     .set_velocity(
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///     )
    ///     .build();
    /// ```
    pub fn set_position(mut self, x: Length, y: Length, z: Length) -> ParticleBuilder {
        self.position = vector3d!(x.get::<meter>(), y.get::<meter>(), z.get::<meter>());
        self
    }

    /// Sets the [`velocity`] of the [`Particle`] as a 3D vector in metres per
    /// second (m/s).
    ///
    /// Can be chained with other setter methods.
    ///
    /// [`velocity`]: Particle::velocity
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uom::si::f64
    ///
    /// let particle = ParticleBuilder::new()
    ///     .set_mass(uom::si::f64::Mass::new::<kilogram>(2.0))
    ///     .set_position(
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///     )
    ///     .set_velocity(
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///     )
    ///     .build();
    /// ```
    pub fn set_velocity(mut self, x: Velocity, y: Velocity, z: Velocity) -> ParticleBuilder {
        self.velocity = vector3d!(
            x.get::<meter_per_second>(),
            y.get::<meter_per_second>(),
            z.get::<meter_per_second>()
        );
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
    /// If a [`Particle`] is directly instantiated without this function,
    /// the automatic identifiers may break.
    ///
    /// [`mass`]: Particle::mass
    /// [`position`]: Particle::position
    /// [`velocity`]: Particle::velocity
    /// [`acceleration`]: Particle::acceleration
    /// [`attached_springs`]: Particle::attached_springs
    /// [`id`]: Particle::id
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uom::si::f64
    ///
    /// let particle = ParticleBuilder::new()
    ///     .set_mass(uom::si::f64::Mass::new::<kilogram>(2.0))
    ///     .set_position(
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///         uom::si::f64::Length::new::<meter>(1.0),
    ///     )
    ///     .set_velocity(
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///         uom::si::f64::Velocity::new::<meter_per_second>(0.5),
    ///     )
    ///     .build();
    /// ```
    pub fn build(self) -> Particle<'a> {
        Particle {
            id: PARTICLE_COUNTER.fetch_add(1, Ordering::SeqCst),
            mass: self.mass,
            position: self.position,
            velocity: self.velocity,
            acceleration: Vector3d::zero(),
            attached_springs: HashMap::new(),
        }
    }
}

/// Counter for the [`id`] property of the [`Spring`] class.
/// Increases by one (1) everytime
///
/// [`id`]: Particle::id
static SPRING_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A spring of a given stiffness connecting two [`Particle`]s.
#[derive(Clone, Debug)]
pub struct Spring {
    /// A unique identifier for this [`Spring`].
    id: usize,
    /// The stiffness of this [`Spring`] in newtons per metre (N/m).
    spring_stiffness: dimension::SpringStiffness,
    /// The resting length of this [`Spring`] in metres (m).
    resting_length: Length,
}

impl Hash for Spring {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Spring {
    fn eq(&self, other: &Self) -> bool {
        self.spring_stiffness == other.spring_stiffness
            && self.resting_length.get::<meter>() == other.resting_length.get::<meter>()
    }
}

impl Eq for Spring {}

impl Spring {
    /// Creates a new [`Spring`]
    /// using the given spring stiffness and resting length
    /// and assigns it the next [`id`].
    ///
    /// As a side effect, increases [`SPRING_COUNTER`] by one (1).
    ///
    /// If a [`Spring`] is directly instantiated without this function,
    /// the automatic identifiers may break.
    ///
    /// [`id`]: Particle::id
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uom::si::f64
    ///
    /// let spring = Spring::new(1.0, 1.0);
    ///
    /// assert_eq!(
    ///     spring,
    ///     Spring {
    ///         id: 0,
    ///         spring_stiffness: 1.0,
    ///         resting_length: 1.0
    ///     }
    /// );
    /// ```
    pub fn new(spring_stiffness: dimension::SpringStiffness, resting_length: Length) -> Spring {
        Self {
            id: SPRING_COUNTER.fetch_add(1, Ordering::SeqCst),
            spring_stiffness,
            resting_length,
        }
    }
}
