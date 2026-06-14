use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::particle::{Particle, ParticleBuilder};

mod particle;


/// Store the parameters given in an input JSON file.
#[derive(Serialize, Deserialize)]
pub struct InputJson {
    dimensions: Vec<u32>,
    mass: f64,
    spring_constant: f64,
}


fn main() {
    let args: Vec<String> = env::args().collect();

    // As a basic test of functionality, copy the input file into the output
    // directory.
    let file_contents = match fs::read_to_string(&args[1]) {
        Ok(file_contents) => file_contents,
        Err(_) => panic!("Error: File `{}` could not be read.", &args[1]),
    };

    let input_json: InputJson = match serde_json::from_str(&file_contents) {
        Ok(input_json) => input_json,
        Err(_) => panic!("Error: File `{}` is malformatted.", &args[1]),
    };

    let mut dimensions = input_json.dimensions.clone();

    if dimensions.len() < 2 {
        println!("Warning: Y dimension not provided. Assuming 1.");
    }

    if dimensions.len() < 3 {
        println!("Warning: Z dimension not provided. Assuming 1.");
    }

    let spring_constant = input_json.spring_constant;
    let particle = ParticleBuilder::new().set_mass(input_json.mass);
}
