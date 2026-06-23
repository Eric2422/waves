use core::panic;
use std::path::{Path, PathBuf};
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
    spring_lengths: [f64; 3],
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

/// Check for various illogical input JSON settings.
fn check_input_json(input_file: &Path, input_json: &mut InputJson) {
    // Check for values that can not accept 0.
    if input_json.time_step_size <= 0.0 {
        panic!(
            "ERROR: The time step size given in {:?} is {} s, but it should be positive.",
            input_file, input_json.time_step_size
        );
    }
    if input_json.mass <= 0.0 {
        panic!(
            "ERROR: The mass given in {:?} is {} kg, but it should be positive.",
            input_file, input_json.mass
        );
    }
    if input_json.spring_constant <= 0.0 {
        panic!(
            "ERROR: The spring constant given in {:?} is {} N/m, but it should be positive.",
            input_file, input_json.spring_constant
        );
    }

    // Set to positive if negative.
    if input_json.damping < 0.0 {
        println!(
            "WARNING: The damping given in {:?} is {} N⋅s⋅m⁻¹, but it should be non-negative.\nAssuming a positive value of {} N⋅s⋅m⁻¹.",
            input_file, input_json.damping, -input_json.damping
        );
        input_json.damping = -input_json.damping;
    }
    if input_json.spring_lengths[0] < 0.0
        || input_json.spring_lengths[1] < 0.0
        || input_json.spring_lengths[2] < 0.0
    {
        println!(
            "WARNING: The springs lengths given in {:?} are {:?} m, but they should be non-negative.\nAssuming positive values of {:?} m.",
            input_file,
            input_json.spring_lengths,
            -vector_3d!(input_json.spring_lengths)
        );
        input_json.spring_lengths[0] = (input_json.spring_lengths[0]).abs();
        input_json.spring_lengths[1] = (input_json.spring_lengths[1]).abs();
        input_json.spring_lengths[2] = (input_json.spring_lengths[2]).abs();
    }
}

/// Calculate the total spring force from the surrounding [`Particle`]s acting
/// upon the [`Particle`] at `particle_indices in `particles`.
fn calculate_spring_force(
    particles: &mut Vec<Vec<Vec<Particle>>>,
    particle_indices: [usize; 3],
    spring_lengths: [f64; 3],
    spring_constant: f64,
) -> Vector3d {
    let center_x = particle_indices[0];
    let center_y = particle_indices[1];
    let center_z = particle_indices[2];

    let center_particle = &particles[center_x][center_y][center_z];

    // Start from the bottom back-left corner of the neighboring particles.
    let start_x = cmp::max(center_x - 1, center_x);
    let start_y = cmp::max(center_y - 1, center_y);
    let start_z = cmp::max(center_z - 1, center_z);

    // Prevent from going out of bounds.
    let end_x = cmp::min(start_x + 3, particles.len());
    let mut end_y: usize;
    let mut end_z: usize;

    // Sum spring force from all neighboring particles.
    let mut total_force = Vector3d(0.0, 0.0, 0.0);
    for x in start_x..end_x {
        end_y = cmp::min(start_y + 3, particles[x].len());

        for y in start_y..end_y {
            end_z = cmp::min(start_z + 3, particles[x][y].len());

            for z in start_z..end_z {
                // Add the force if it is not the center particle.
                if x != center_x && y != center_y && z != center_z {
                    // Get the current, stretched vector between the particles.
                    let distance_vector = center_particle.position - particles[x][y][z].position;
                    // Calculate the original resting distance.
                    let rest_distance = Vector3d(
                        (x - center_x) as f64 * spring_lengths[0],
                        (y - center_y) as f64 * spring_lengths[1],
                        (z - center_z) as f64 * spring_lengths[2],
                    )
                    .get_magnitude();

                    // Apply Hooke's Law.
                    total_force += -spring_constant
                        * (distance_vector.get_magnitude() - rest_distance)
                        * distance_vector.get_normalized();
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
                let mut total_force = calculate_spring_force(
                    particles,
                    [x, y, z],
                    input_json.spring_lengths,
                    input_json.spring_constant,
                );

                // Add the driving force to particles at the start end.
                if x == 0 {
                    total_force += driving_force;
                }

                particles[x][y][z].acceleration = total_force / particles[x][y][z].mass;
            }
        }
    }

    // Update position separately to prevent it from affecting spring force
    // calculations.
    for x in 0..particles.len() {
        for y in 0..particles[x].len() {
            for z in 0..particles[x][y].len() {
                let acceleration = particles[x][y][z].acceleration;
                particles[x][y][z].velocity += acceleration * input_json.time_step_size;

                let velocity = particles[x][y][z].velocity;
                particles[x][y][z].position += velocity * input_json.time_step_size;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!(
            "ERROR: No input file provided. Usage: `./target/debug/longitudinal_waves.exe input/<file name>`"
        );
    }

    let input_file = Path::new(&args[1]);

    // Attempt to retreive the contents of the file.
    let file_contents = match fs::read_to_string(input_file) {
        Ok(file_contents) => file_contents,
        Err(_) => panic!(
            "ERROR: File `{:?}` could not be read. Check if the file exists.",
            input_file
        ),
    };

    // Attempt to parse the file into usable data.
    let mut input_json: InputJson = match serde_json::from_str(&file_contents) {
        Ok(input_json) => input_json,
        Err(_) => panic!(
            "ERROR: File `{}` is malformatted. Check to make sure that it is properly formatted as given by the sample.",
            &args[1]
        ),
    };

    check_input_json(input_file, &mut input_json);

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
                            (x as f64) * input_json.spring_lengths[0],
                            (y as f64) * input_json.spring_lengths[1],
                            (z as f64) * input_json.spring_lengths[2],
                        )
                        .build(),
                );
            }
        }
    }

    for i in 0..input_json.total_time_steps {
        let current_time = (i as f64) * input_json.time_step_size;

        // fs::write(path, contents);

        update_particles(&mut particles, &input_json, current_time);
    }
}
