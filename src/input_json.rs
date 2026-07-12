//! Handles the information from [`serde`] deserializing the input JSON files.

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uom::{
    fmt::DisplayStyle::Abbreviation,
    si::{
        angle::radian,
        angular_velocity::radian_per_second,
        f64::{Angle, AngularVelocity, Force, Length, Mass, Time},
        force, length,
        mass_rate::kilogram_per_second,
        surface_tension::newton_per_meter,
        time::second,
    },
};

use crate::dimension;


/// Stores the driving parameters as part of [`InputJson`].
#[derive(Serialize, Deserialize)]
pub struct DrivingParameters {
    /// The amplitude of the driving force as a 3D vector
    /// measured in newtons (N).
    pub amplitude: [Force; 3],
    /// The angular frequency of the driving force
    /// in radians per second (rad/s).
    pub angular_frequency: AngularVelocity,
    /// The phase of the driving force in radians (rad).
    pub phase: Angle,
}

/// Stores the parameters given in an input JSON file.
#[derive(Serialize, Deserialize)]
pub struct InputJson {
    /// The total number of time steps to run.
    pub total_time_steps: u32,
    /// Size of each time step in seconds (s).
    pub time_step_size: Time,
    /// The number of [`Particle`]s in each direction: x, y, and z.
    ///
    /// [`Particle`]: crate::particle::Particle
    pub dimensions: [usize; 3],
    /// The distance between [`Particle`]s in each direction.
    /// Measured in meters (m).
    ///
    /// [`Particle`]: crate::particle::Particle
    pub particle_distances: [Length; 3],
    ///
    /// The mass of each individual [`Particle`] in kilograms (kg).
    ///
    /// [`Particle`]: crate::particle::Particle
    pub mass: Mass,
    /// The spring constant between each pair of [`Particle`]s,
    /// measured in newtons per meter (N/m).
    ///
    /// [`Particle`]: crate::particle::Particle
    pub spring_constant: dimension::SpringConstant,
    /// The damping coefficient of the [`Spring`]s
    /// in newton-seconds per meter (N⋅s⋅m⁻¹)
    /// or dimensionally equivalently in kilograms per second (kg/s).
    ///
    /// [`Spring`]: crate::particle::Spring
    pub damping: dimension::ViscousDamping,
    pub driving: DrivingParameters,
}

impl Display for InputJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\
Total time steps: {}
Time step size: {}
Particle distances: ({}, {}, {}) {}
Dimensions: {:?}
Spring constant: {}
Damping: {}
Driving parameters:
    Amplitude: ({}, {}, {}) {}
    Angular frequency: {}
    Phase: {}",
            self.total_time_steps,
            self.time_step_size.into_format_args(second, Abbreviation),
            self.particle_distances[0].get::<length::meter>(),
            self.particle_distances[1].get::<length::meter>(),
            self.particle_distances[1].get::<length::meter>(),
            length::Units::meter.abbreviation(),
            self.dimensions,
            self.spring_constant
                .into_format_args(newton_per_meter, Abbreviation),
            self.damping
                .into_format_args(kilogram_per_second, Abbreviation),
            self.driving.amplitude[0].get::<force::newton>(),
            self.driving.amplitude[1].get::<force::newton>(),
            self.driving.amplitude[2].get::<force::newton>(),
            force::Units::newton.abbreviation(),
            self.driving
                .angular_frequency
                .into_format_args(radian_per_second, Abbreviation),
            self.driving.phase.into_format_args(radian, Abbreviation)
        )
    }
}
