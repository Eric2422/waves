use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::particle::ParticleBuilder;

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

    // If the number of dimensions provided is less than 3, pad the end with 1
    // (i.e., there is 1 particle in that dimension).
    let mut dimensions: [u32; 3] = [1, 1, 1];
    if input_json.dimensions.len() < 3 {
        println!("Warning: Less than 3 dimensions provided. Assuming 1 for missing dimensions.");
    }
    for i in 0..2 {
        if input_json.dimensions.len() > i + 1 {
            dimensions[i] = input_json.dimensions[i]
        }
    }

    let spring_constant = input_json.spring_constant;
    let particle = ParticleBuilder::new().set_mass(input_json.mass).build();
}
