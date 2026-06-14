use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::particle::ParticleBuilder;

mod particle;


/// Store the parameters given in the input file.
#[derive(Serialize, Deserialize)]
pub struct InputFile {
    dimensions: Vec<u32>,
    mass: f64,
    spring_constant: f64,
}


fn main() {
    let args: Vec<String> = env::args().collect();

    // As a basic test of functionality, copy the input file into the output
    // directory.
    let file_contents = match fs::read_to_string(&args[1]) {
        Ok(it) => it,
        Err(_err) => panic!("File `{}` could not be read.", &args[1]),
    };

    let input_file: InputFile = match serde_json::from_str(&file_contents) {
        Ok(it) => it,
        Err(_err) => panic!("File `{}` is malformatted.", &args[1]),
    };

    let spring_constant = input_file.spring_constant;
    let particle = ParticleBuilder::new().set_mass(input_file.mass);
}
