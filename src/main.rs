mod dimension;
mod input_json;
mod particle;
mod vector3d;

use std::{
    cmp, env,
    fmt::Write as _,
    fs,
    io::Write,
    path::{Path, PathBuf},
    result,
};

use uom::{
    ConstZero,
    fmt::DisplayStyle::Abbreviation,
    si::{
        angle::radian,
        f64::{Length, Mass, MassRate, Time},
        force::newton,
        length::meter,
        mass::kilogram,
        mass_rate::kilogram_per_second,
        surface_tension::newton_per_meter,
        time::second,
    },
};

use crate::{
    dimension::SpringConstant,
    input_json::InputJson,
    particle::{Particle, ParticleBuilder},
    vector3d::Vector3d,
};


/// The [`str`] representation of the output directory.
/// A [`Path`] would be easier to work with,
/// but [`Path`]s can not be instantiated statically.
static OUTPUT_DIR_STRING: &str = "output";

/// Checks for various illogical [input JSON] settings.
/// Corrects [time step size], [mass], and [spring constant] to
/// [`f64::MIN_POSITIVE`] if 0.0.
/// Sets [time step size], [mass], [spring constant], [damping], and
/// [particle distances] to their absolute values if negative.
///
/// If any such corrections are made, return [`false`].
/// Else, return [`true`].
///
/// [input JSON]: InputJson
/// [time step size]: InputJson::time_step_size
/// [mass]: InputJson::mass
/// [spring constant]: InputJson::spring_constant
/// [damping]: InputJson::damping
/// [particle distances]: InputJson::particle_distances
/// [`false`]: bool
/// [`true`]: bool
///
/// # Examples
///
/// ```rust
/// // Invalidated by the negative mass.
/// assert_eq!(
///     false,
///     InputJson {
///         total_time_steps: 120,
///         time_step_size: Time::new::<second>(0.5),
///         dimensions: [5, 5, 5],
///         particle_distances: [
///             Length::new::<meter>(1.0),
///             Length::new::<meter>(1.0),
///             Length::new::<meter>(1.0)
///         ],
///         mass: Mass::new::<kilogram>(-1.0),
///         spring_constant: SurfaceTension::new::<newton_per_meter>(1.0),
///         damping: MassRate::new::<kilogram_per_second>(1.0),
///         driving: DrivingParameters {
///             amplitude: [
///                 Force::new::<newton>(1.0),
///                 Force::new::<newton>(0.0),
///                 Force::new::<newton>(0.0)
///             ],
///             angular_frequency: AngularVelocity::new::<radian_per_second>(1.0),
///             phase: Angle::new::<radain>(0.0)
///         }
///     }
/// );
/// ```
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
            "Warning: The time step size given in {input_file_path:?} is {} s, but it should be positive.
Assuming a positive value of {} s.",
            input_json.time_step_size.into_format_args(second, Abbreviation),
            (-input_json.time_step_size).into_format_args(second, Abbreviation)
        );
        input_json.time_step_size = -input_json.time_step_size;
        passed_all_checks = false;
    }
    if input_json.mass < Mass::ZERO {
        println!(
            "Warning: The mass given in {input_file_path:?} is {} kg, but it should be positive.
Assuming a positive value of {} kg.",
            input_json.mass.into_format_args(kilogram, Abbreviation),
            (-input_json.mass).into_format_args(kilogram, Abbreviation)
        );
        input_json.mass = -input_json.mass;
        passed_all_checks = false;
    }
    if input_json.particle_distances[0] < Length::ZERO
        || input_json.particle_distances[1] < Length::ZERO
        || input_json.particle_distances[2] < Length::ZERO
    {
        println!(
            "Warning: The springs lengths given in {input_file_path:?} are ({}, {}, {}) m, but they should be non-negative.",
            input_json.particle_distances[0].get::<meter>(),
            input_json.particle_distances[1].get::<meter>(),
            input_json.particle_distances[2].get::<meter>()
        );

        input_json.particle_distances[0] = (input_json.particle_distances[0]).abs();
        input_json.particle_distances[1] = (input_json.particle_distances[1]).abs();
        input_json.particle_distances[2] = (input_json.particle_distances[2]).abs();
        println!(
            "Assuming positive values of ({}, {}, {}) m.",
            input_json.particle_distances[0].get::<meter>(),
            input_json.particle_distances[1].get::<meter>(),
            input_json.particle_distances[2].get::<meter>()
        );

        passed_all_checks = false;
    }
    if input_json.spring_constant < SpringConstant::ZERO {
        println!(
            "Warning: The spring constant given in {input_file_path:?} is {} N/m, but it should be positive.
Assuming a positive value of {} N/m.",
            input_json.spring_constant.into_format_args(newton_per_meter, Abbreviation),
            (-input_json.spring_constant).into_format_args(newton_per_meter, Abbreviation)
        );
        input_json.spring_constant = -input_json.spring_constant;
        passed_all_checks = false;
    }
    if input_json.damping < MassRate::ZERO {
        println!(
            "Warning: The damping given in {input_file_path:?} is {} N⋅s⋅m⁻¹, but it should be non-negative.
Assuming a positive value of {} N⋅s⋅m⁻¹.",
            input_json.damping.into_format_args(kilogram_per_second, Abbreviation),
            (-input_json.damping).into_format_args(kilogram_per_second, Abbreviation)
        );
        input_json.damping = -input_json.damping;
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
    let start_z = if center_z == 0 {
        center_z
    } else {
        center_z - 1
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
                        (x as f64 - center_x as f64) * spring_lengths[0].get::<meter>(),
                        (y as f64 - center_y as f64) * spring_lengths[1].get::<meter>(),
                        (z as f64 - center_z as f64) * spring_lengths[2].get::<meter>()
                    )
                    .get_magnitude();

                    // Apply Hooke's Law.
                    spring_force += -spring_constant.get::<newton_per_meter>()
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
/// Returns a [`String`] of the [`Particle`]'s states before updating.
///
/// [`acceleration`]: Particle::acceleration
/// [`velocity`]: Particle::velocity
/// [`position`]: Particle::position
fn update_particles(
    particles: &mut Vec<Vec<Vec<Particle>>>,
    input_json: &InputJson,
    current_time: Time,
) -> result::Result<String, String> {
    // Calculate the current force given by a sinusoidal driving force.
    let driving_force = vector3d!(
        input_json.driving.amplitude[0].get::<newton>(),
        input_json.driving.amplitude[1].get::<newton>(),
        input_json.driving.amplitude[2].get::<newton>()
    ) * ((input_json.driving.angular_frequency * current_time).value
        + input_json.driving.phase.get::<radian>())
    .cos();

    let mut output_string = String::new();

    // Apply forces to all particles.
    for x in 0..particles.len() {
        for y in 0..particles[x].len() {
            for z in 0..particles[x][y].len() {
                // To avoid having to loop through again,
                // output the `Particle` states to a `String`.
                match writeln!(&mut output_string, "{}", particles[x][y][z]) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(format!(
                            "ERROR: Failed to write Particle {} at indices ({x}, {y}, {z}) to the output string!",
                            particles[x][y][z].id
                        ));
                    }
                }

                let mut total_force = calculate_spring_force(
                    particles,
                    [x, y, z],
                    input_json.particle_distances,
                    input_json.spring_constant,
                );

                total_force -=
                    input_json.damping.get::<kilogram_per_second>() * particles[x][y][z].velocity;

                // Add the driving force to particles at the start end.
                if x == 0 {
                    total_force += driving_force;
                }

                particles[x][y][z].acceleration =
                    total_force / particles[x][y][z].mass.get::<kilogram>();
            }
        }

        // Blank line just to make the output easier to understand.
        match writeln!(&mut output_string) {
            Ok(_) => {}
            Err(_) => {
                return Err(format!(
                    "ERROR: Failed to write newline after x-index {x} to the output string!"
                ));
            }
        };
    }

    // Update position separately to prevent it from affecting spring force
    // calculations.
    for x in 0..particles.len() {
        for y in 0..particles[x].len() {
            for z in 0..particles[x][y].len() {
                let acceleration = particles[x][y][z].acceleration;
                particles[x][y][z].velocity +=
                    acceleration * input_json.time_step_size.get::<second>();

                let velocity = particles[x][y][z].velocity;
                particles[x][y][z].position += velocity * input_json.time_step_size.get::<second>();
            }
        }
    }

    Ok(output_string)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!(
            "ERROR: No input file provided. Usage: `./target/debug/longitudinal_waves!exe input/<file name>`."
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
                panic!("ERROR: The input file path {input_file_path:?} is an invalid OS string!")
            })
            .to_str()
            .unwrap_or_else(|| {
                panic!("ERROR: The input file path {input_file_path:?} is an invalid string!")
            }),
    ]
    .iter()
    .collect();

    // Attempt to retreive the contents of the file.
    let file_contents = fs::read_to_string(input_file_path).unwrap_or_else(|_| {
        panic!("ERROR: The input file {input_file_path:?} could not be read! Try checking if it exists.")
    });

    // Attempt to parse the file into usable data.
    let mut input_json: InputJson = serde_json::from_str(&file_contents).unwrap_or_else(|_| {
        panic!(
            "ERROR: The input file {input_file_path:?} is malformatted!
Try checking that it is properly formatted as given in \"README.md\".",
        )
    });

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
                "ERROR: Unable to create or open the output file {output_file_path:?}!
Try checking if the \"output/\" directory exists."
            );
        });
    // Clear the file before writing to it.
    // For some reason, opening a file with truncate() seems to result in an error.
    output_file.set_len(0).unwrap_or_else(|_| {
        println!("Warning: Failed to clear the output file {output_file_path:?}.")
    });
    // Add information about the input JSON file to the top.
    writeln!(
        output_file,
        "\
Input JSON: {input_file_path:?}
{input_json}
"
    )
    .unwrap_or_else(|_| {
        println!("Warning: Failed to write to the output file {output_file_path:?}.")
    });

    // Create a grid of identical particles.
    let mut particles: Vec<Vec<Vec<Particle>>> = Vec::new();
    for x in 0..input_json.dimensions[0] {
        particles.push(Vec::new());

        for y in 0..input_json.dimensions[1] {
            particles[x].push(Vec::new());

            for z in 0..input_json.dimensions[2] {
                // Only apply initial velocity to the first x-layer,
                // i.e., the driven particles.
                particles[x][y].push(if x == 0 {
                    ParticleBuilder::new(input_json.mass)
                        .set_position(
                            (x as f64) * input_json.particle_distances[0],
                            (y as f64) * input_json.particle_distances[1],
                            (z as f64) * input_json.particle_distances[2],
                        )
                        .set_velocity(
                            input_json.initial_velocity[0],
                            input_json.initial_velocity[1],
                            input_json.initial_velocity[2],
                        )
                        .build()
                } else {
                    ParticleBuilder::new(input_json.mass)
                        .set_position(
                            (x as f64) * input_json.particle_distances[0],
                            (y as f64) * input_json.particle_distances[1],
                            (z as f64) * input_json.particle_distances[2],
                        )
                        .build()
                });
            }
        }
    }

    // Run the time steps.
    let mut current_time = Time::ZERO;
    for i in 0..=input_json.total_time_steps {
        writeln!(
            output_file,
            "
Time step {i}, t = {}",
            current_time.into_format_args(second, Abbreviation)
        )
        .unwrap_or_else(|_| {
            println!("Warning: Failed to write to the output file {output_file_path:?}.");
        });

        current_time += input_json.time_step_size;

        // Calculate and print the particles.
        match update_particles(&mut particles, &input_json, current_time) {
            Ok(output_string) => write!(output_file, "{output_string}").unwrap_or_else(|_| {
                println!(
                    "Warning: Failed to write time step {i} (t = {}) into the output file {output_file_path:?}.",
                    current_time.into_format_args(second, Abbreviation)
                );
            }),
            Err(error) => panic!("{error}"),
        };
    }
}
