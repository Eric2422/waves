use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::particle::{Particle, ParticleBuilder};

mod particle;


/// Store the parameters given in an input JSON file.
#[derive(Serialize, Deserialize)]
pub struct InputJson {
    dimensions: [usize; 3],
    distance: [f64; 3],
    mass: f64,
    spring_constant: f64,
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
}
