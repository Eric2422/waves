use std::{
    cmp, env, fs,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use uom::{
    ConstZero,
    si::{
        f64::{Angle, AngularVelocity, Force, Length, Mass, MassRate, SurfaceTension, Time},
        mass::kilogram,
        surface_tension::newton_per_meter,
        time::second,
    },
};

mod particle;
use crate::particle::{Particle, ParticleBuilder};
mod vector3d;
use crate::vector3d::Vector3d;


/// Alias for [`SurfaceTension`] to more accurately describe spring constants
/// rather than surface tension, which are dimensionally equivalent.
type SpringConstant = SurfaceTension;
/// Alias for [`MassRate`]
/// because the damping coefficient and rate of mass change
/// are dimensionally equivalent,
/// i.e., newton-seconds per meter or kilograms per second.
type ViscousDamping = MassRate;


/// The [`str`] representation of the output directory.
/// A [`Path`] would be easier to work with,
/// but [`Path`]s can not be instantiated statically.
static OUTPUT_DIR_STRING: &str = "output";


/// Store the parameters given in an input JSON file.
#[derive(Serialize, Deserialize)]
pub struct InputJson {
    /// Size of each time step in seconds (s).
    time_step_size: Time,
    /// The total number of time steps to run.
    total_time_steps: u32,
    /// The number of [`Particle`]s in each direction: x, y, and z.
    dimensions: [usize; 3],
    /// The distance between [`Particle`]s in each direction.
    /// Measured in meters (m).
    spring_lengths: [Length; 3],
    /// The mass of each individual [`Particle`] in kilograms (kg).
    mass: Mass,
    /// The spring constant between each pair of particles,
    /// measured in newtons per meter (N/m).
    spring_constant: SpringConstant,
    /// The damping coefficient of the springs
    /// in newton-seconds per meter (N⋅s⋅m⁻¹).
    damping: ViscousDamping,
    /// The amplitude of the driving force as a 3D vector
    /// measured in newtons (N).
    driving_amplitude: [Force; 3],
    /// The angular frequency of the driving force
    /// in radians per second (rad/s).
    driving_angular_frequency: AngularVelocity,
    /// The phase shift of the driving force, which is a dimensionless value.
    driving_phase: Angle,
}

/// Checks for various illogical input JSON settings.
/// Corrects time step size, mass, and spring constant to [`f64::MIN_POSITIVE`]
/// if 0.0.
/// Sets time step size, mass, spring constant, damping, and spring lengths to
/// their absolute values if negative.
///
/// If any such corrections are made, return [`false`]. Else, return [`true`].
///
/// [`false`]: bool
/// [`true`]: bool
fn check_input_json(input_file_path: &Path, input_json: &mut InputJson) -> bool {
    let mut passed_all_checks = true;

    // Check for values that can not accept 0.
    // If so, set it to be the minimum positive value.
    if input_json.time_step_size == Time::ZERO {
        println!(
            "Warning: The time step size given in {input_file_path:?} is 0.0 s, but it should be non-zero.
Setting to the smallest positive value {} s.",
            f64::MIN_POSITIVE
        );
        input_json.time_step_size = Time::new::<second>(f64::MIN_POSITIVE);
        passed_all_checks = false;
    }
    if input_json.mass == Mass::ZERO {
        println!(
            "Warning: The mass given in {input_file_path:?} is 0.0 kg, but it should be non-zero.
Setting to the smallest positive value {} kg.",
            f64::MIN_POSITIVE
        );
        input_json.mass = Mass::new::<kilogram>(f64::MIN_POSITIVE);
        passed_all_checks = false;
    }
    if input_json.spring_constant == SpringConstant::ZERO {
        println!(
            "Warning: The spring constant given in {input_file_path:?} is 0.0 N/m, but it should be non-zero.
Setting to the smallest positive value {} N/m.",
            f64::MIN_POSITIVE
        );
        input_json.spring_constant = SpringConstant::new::<newton_per_meter>(f64::MIN_POSITIVE);
        passed_all_checks = false;
    }

    // For values that cannot accept a negative value, flip it to be positive.
    if input_json.time_step_size < Time::ZERO {
        println!(
            "Warning: The time step size given in {input_file_path:?} is {:?} s, but it should be positive.
Assuming a positive value of {:?} s.",
            input_json.time_step_size,
            -input_json.time_step_size
        );
        input_json.time_step_size = -input_json.time_step_size;
        passed_all_checks = false;
    }
    if input_json.mass < Mass::ZERO {
        println!(
            "Warning: The mass given in {input_file_path:?} is {:?} kg, but it should be positive.
Assuming a positive value of {:?} kg.",
            input_json.mass, -input_json.mass
        );
        input_json.mass = -input_json.mass;
        passed_all_checks = false;
    }
    if input_json.spring_constant < SpringConstant::ZERO {
        println!(
            "Warning: The spring constant given in {input_file_path:?} is {:?} N/m, but it should be positive.
Assuming a positive value of {:?} N/m.",
            input_json.spring_constant,
            -input_json.spring_constant
        );
        input_json.spring_constant = -input_json.spring_constant;
        passed_all_checks = false;
    }
    if input_json.damping < MassRate::ZERO {
        println!(
            "Warning: The damping given in {input_file_path:?} is {:?} N⋅s⋅m⁻¹, but it should be non-negative.
Assuming a positive value of {:?} N⋅s⋅m⁻¹.",
            input_json.damping, -input_json.damping
        );
        input_json.damping = -input_json.damping;
        passed_all_checks = false;
    }
    if input_json.spring_lengths[0] < Length::ZERO
        || input_json.spring_lengths[1] < Length::ZERO
        || input_json.spring_lengths[2] < Length::ZERO
    {
        println!(
            "Warning: The springs lengths given in {input_file_path:?} are {:?} m, but they should be non-negative.",
            input_json.spring_lengths
        );

        input_json.spring_lengths[0] = (input_json.spring_lengths[0]).abs();
        input_json.spring_lengths[1] = (input_json.spring_lengths[1]).abs();
        input_json.spring_lengths[2] = (input_json.spring_lengths[2]).abs();
        println!(
            "\nAssuming positive values of {:?} m.",
            input_json.spring_lengths
        );

        passed_all_checks = false;
    }

    passed_all_checks
}

/// Calculates the total spring force from the surrounding [`Particle`]s acting
/// upon the [`Particle`] at `particle_indices in `particles`.
fn calculate_spring_force(
    particles: &mut Vec<Vec<Vec<Particle>>>,
    particle_indices: [usize; 3],
    spring_lengths: [Length; 3],
    spring_constant: SpringConstant,
) -> Vector3d {
    let center_x = particle_indices[0];
    let center_y = particle_indices[1];
    let center_z = particle_indices[2];

    let center_particle = &particles[center_x][center_y][center_z];

    // Start from the bottom back-left corner of the neighboring particles.
    let start_x = if center_x == 0 {
        center_x
    } else {
        center_x - 1
    };
    let start_y = if center_y == 0 {
        center_y
    } else {
        center_y - 1
    };
    let start_z = if center_y == 0 {
        center_y
    } else {
        center_y - 1
    };

    // Prevent from going out of bounds.
    let end_x = cmp::min(start_x + 3, particles.len());

    // Sum spring force from all neighboring particles.
    let mut spring_force = vector3d!(0.0, 0.0, 0.0);
    for x in start_x..end_x {
        let end_y = cmp::min(start_y + 3, particles[x].len());

        for y in start_y..end_y {
            let end_z = cmp::min(start_z + 3, particles[x][y].len());

            for z in start_z..end_z {
                // Add the force if it is not the center particle.
                if particles[x][y][z] != *center_particle {
                    // Get the current, stretched vector between the particles.
                    let distance_vector = center_particle.position - particles[x][y][z].position;
                    // Calculate the resting length.
                    let resting_length = vector3d!(
                        (x as f64 - center_x as f64) * spring_lengths[0].value,
                        (y as f64 - center_y as f64) * spring_lengths[1].value,
                        (z as f64 - center_z as f64) * spring_lengths[2].value
                    )
                    .get_magnitude();

                    // Apply Hooke's Law.
                    spring_force += -spring_constant.value
                        * (distance_vector.get_magnitude() - resting_length)
                        * distance_vector.get_normalized();
                }
            }
        }
    }

    spring_force
}

/// Updates the current [`acceleration`], [`velocity`], and [`position`] of the
/// [`Particle`]s.
///
/// If the value in `output_file` is [`None`], no output will be written.
/// If it is [`Some`], output will written to the given [`File`].
///
/// [`acceleration`]: Particle::acceleration
/// [`velocity`]: Particle::velocity
/// [`position`]: Particle::position
/// [`File`]: fs::File
fn update_particles(
    particles: &mut Vec<Vec<Vec<Particle>>>,
    input_json: &InputJson,
    current_time: Time,
    mut output_file: Option<&mut fs::File>,
) {
    // Calculate the current force given by a sinusoidal driving force.
    let driving_force = vector3d!(
        input_json.driving_amplitude[0].value,
        input_json.driving_amplitude[1].value,
        input_json.driving_amplitude[2].value
    ) * ((input_json.driving_angular_frequency * current_time).value
        + input_json.driving_phase.value)
        .cos();

    // Apply forces to all particles.
    for x in 0..particles.len() {
        for y in 0..particles[x].len() {
            for z in 0..particles[x][y].len() {
                // To avoid having to loop through again,
                // output the `Particle` states to a file.
                match output_file {
                    Some(ref mut output_file) => {
                        writeln!(output_file, "{}", particles[x][y][z]).unwrap_or_else(|_| {})
                    }
                    None => {}
                }

                let mut total_force = calculate_spring_force(
                    particles,
                    [x, y, z],
                    input_json.spring_lengths,
                    input_json.spring_constant,
                );

                total_force -= input_json.damping.value * particles[x][y][z].velocity;

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
                particles[x][y][z].velocity += acceleration * input_json.time_step_size.value;

                let velocity = particles[x][y][z].velocity;
                particles[x][y][z].position += velocity * input_json.time_step_size.value;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!(
            "ERROR: No input file provided. Usage: `./target/debug/longitudinal_waves!exe input/<file name>`"
        );
    }

    let input_file_path = Path::new(&args[1]);
    // Automatically generate the output file to have the same name
    // but to be a .txt file in `output/`.
    let output_file_path: PathBuf = [
        OUTPUT_DIR_STRING,
        input_file_path
            .with_extension("txt")
            .file_name()
            .unwrap_or_else(|| {
                panic!("ERROR: Input file name {input_file_path:?} is an invalid OS string!")
            })
            .to_str()
            .unwrap_or_else(|| {
                panic!("ERROR: Input file name {input_file_path:?} is an invalid string!")
            }),
    ]
    .iter()
    .collect();

    // Attempt to retreive the contents of the file.
    let file_contents = fs::read_to_string(input_file_path).unwrap_or_else(|_| {
        panic!(
            "ERROR: File `{input_file_path:?}` could not be read. Try checking if the file exists!"
        )
    });

    // Attempt to parse the file into usable data.
    let mut input_json: InputJson= serde_json::from_str(&file_contents)
        .unwrap_or_else(
            |_| panic!(
                "ERROR: File `{}` is malformatted. Check to make sure that it is properly formatted as given by the sample!",
                &args[1]
            )
        );

    // Check for invalid values and correct them if found.
    if check_input_json(input_file_path, &mut input_json) {
        println!("{input_file_path:?} passed all checks.");
    }

    // Try to create or access the output file.
    let mut output_file = fs::File::options()
        .write(true)
        .create(true)
        .open(&output_file_path)
        .unwrap_or_else(|_| {
            panic!(
                "ERROR: Unable to create or open {:?}!
Try checking if the output/ directory exists.",
                "test.txt"
            );
        });
    writeln!(output_file, "Input: {input_file_path:?}")
        .unwrap_or_else(|_| println!("Warning: Failed to write to {output_file_path:?}."));

    // Create a grid of identical particles.
    let mut particles: Vec<Vec<Vec<Particle>>> = Vec::new();
    for x in 0..input_json.dimensions[0] {
        particles.push(Vec::new());

        for y in 0..input_json.dimensions[1] {
            particles[x].push(Vec::new());

            for z in 0..input_json.dimensions[2] {
                particles[x][y].push(
                    ParticleBuilder::new(input_json.mass.value)
                        .set_position(
                            (x as f64) * input_json.spring_lengths[0].value,
                            (y as f64) * input_json.spring_lengths[1].value,
                            (z as f64) * input_json.spring_lengths[2].value,
                        )
                        .build(),
                );
            }
        }
    }

    // Run the time steps.
    let mut current_time = Time::ZERO;
    for i in 0..=input_json.total_time_steps {
        writeln!(output_file, "\nTime step {i}, t = {current_time:?}").unwrap_or_else(|_| {
            println!("WARNING: Failed to write to {output_file_path:?}.");
        });

        current_time += input_json.time_step_size;

        // Calculate and print the particles.
        update_particles(
            &mut particles,
            &input_json,
            current_time,
            Some(&mut output_file),
        );
    }
}
