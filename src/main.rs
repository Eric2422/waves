use std::{env, fs};

use serde::{Deserialize, Serialize};

mod vector3d;
mod particle;
use crate::particle::{Particle, ParticleBuilder};

/// Store the parameters given in an input JSON file.
#[derive(Serialize, Deserialize)]
pub struct InputJson {
    /// Size of each time step in seconds (s).
    time_step_size: f64,
    /// The total number of time steps to run.
    total_time_steps: u32,
    /// The number of [`Particle`]s in each direction: x, y, and z.
    dimensions: [usize; 3],
    /// The distance between [`Particle`]s in each direction.
    /// Measured in meters (m).
    distance: [f64; 3],
    /// The mass of each individual [`Particle`] in kilograms (kg).
    mass: f64,
    /// The spring constant between each pair of particles,
    /// measured in newtons per meter (N/m).
    spring_constant: f64,
    /// The damping coefficient of the springs
    /// in newton-seconds per meter (N⋅s⋅m⁻¹).
    damping: f64,
    /// The amplitude of the driving force as a 3D vector
    /// measured in newtons (N).
    driving_amplitude: [f64; 3],
    /// The angular frequency of the driving force
    /// in radians per second (rad/s).
    driving_frequency: f64,
    /// The phase shift of the driving force, which is a dimensionless value.
    driving_phase: f64,
}

/// Update the current [`acceleration`], [`velocity`], and [`position`] of the
/// [`Particle`]s.
///
/// [`acceleration`]: Particle::acceleration
/// [`velocity`]: Particle::velocity
/// [`position`]: Particle::position
fn update_particles(
    particles: &mut Vec<Vec<Vec<Particle>>>,
    input_json: &InputJson,
    current_time: f64,
) {
    // Calculate the current force given by a sinusoidal driving force.
    let current_force = vector3d::multiply_array_by_scalar(
        input_json.driving_amplitude,
        (input_json.driving_frequency * current_time + input_json.driving_phase).cos(),
    );

    // Set the acceleration based on the driving force.
    // The forces from any springs will be added later.
    for y in 0..particles[0].len() {
        for z in 0..particles[0][y].len() {
            particles[0][y][z].acceleration =
                vector3d::divide_array_by_scalar(current_force, particles[0][y][z].mass);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Attempt to retreive the contents of the file.
    let file_contents = match fs::read_to_string(&args[1]) {
        Ok(file_contents) => file_contents,
        Err(_) => panic!("Error: File `{}` could not be read.", &args[1]),
    };

    // Attempt to parse the file into usable data.
    let input_json: InputJson = match serde_json::from_str(&file_contents) {
        Ok(input_json) => input_json,
        Err(_) => panic!("Error: File `{}` is malformatted.", &args[1]),
    };

    // Create a grid of identical particles.
    let mut particles: Vec<Vec<Vec<Particle>>> = Vec::new();
    for x in 0..input_json.dimensions[0] {
        particles.push(Vec::new());

        for y in 0..input_json.dimensions[1] {
            particles[x].push(Vec::new());

            for z in 0..input_json.dimensions[2] {
                particles[x][y].push(
                    ParticleBuilder::new()
                        .set_mass(input_json.mass)
                        .set_position(
                            (x as f64) * input_json.distance[0],
                            (y as f64) * input_json.distance[1],
                            (z as f64) * input_json.distance[2],
                        )
                        .build(),
                );
            }
        }
    }

    for i in 0..input_json.total_time_steps {
        let time = (i as f64) * input_json.time_step_size;
    }
}
