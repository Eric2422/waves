use std::{cmp, env, fs};

use serde::{Deserialize, Serialize};

mod particle;
use crate::particle::{Particle, ParticleBuilder};
mod vector3d;
use crate::vector3d::Vector3d;

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

/// Calculate the total spring force from the surrounding [`Particle`]s acting
/// upon the [`Particle`] at `particle_indices in `particles`.
fn calculate_spring_force(
    particles: &mut Vec<Vec<Vec<Particle>>>,
    particle_indices: [usize; 3],
    spring_constant: f64,
) -> Vector3d {
    let center_x = particle_indices[0];
    let center_y = particle_indices[1];
    let center_z = particle_indices[2];

    let center_particle = &particles[center_x][center_y][center_z];

    // Start from the bottom back-left corner of the neighboring particles.
    let start_x = if center_x > 0 { center_x - 1 } else { center_x };
    let start_y = if center_y > 0 { center_y - 1 } else { center_y };
    let start_z = if center_z > 0 { center_z - 1 } else { center_z };

    // Prevent from going out of bounds.
    let end_x = cmp::min(start_x + 3, particles.len());

    // Sum spring force from all neighboring particles.
    let mut total_force = Vector3d(0.0, 0.0, 0.0);
    for x in start_x..end_x {
        let end_y = cmp::min(start_y + 3, particles[x].len());

        for y in start_y..end_y {
            let end_z = cmp::min(start_z + 3, particles[x][y].len());

            for z in start_z..end_z {
                // Add the force if it is not the center particle.
                total_force += if *center_particle == particles[x][y][z] {
                    Vector3d::zero()
                } else {
                    -spring_constant * (center_particle.position - particles[x][y][z].position)
                }
            }
        }
    }

    total_force
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
    let driving_force = vector_3d!(input_json.driving_amplitude)
        * (input_json.driving_frequency * current_time + input_json.driving_phase).cos();

    // Apply forces to all particles.
    for x in 0..particles.len() {
        for y in 0..particles[x].len() {
            for z in 0..particles[x][y].len() {
                let mut total_force =
                    calculate_spring_force(particles, [x, y, z], input_json.spring_constant);

                // Add the driving force to particles at the start end.
                if x == 0 {
                    total_force += driving_force;
                }
            }
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
        let current_time = (i as f64) * input_json.time_step_size;

        update_particles(&mut particles, &input_json, current_time);
    }
}
