use serde::{Deserialize, Serialize};
use std::{env, fs};

use crate::particle::ParticleBuilder;

mod particle;


/// Store the parameters given in an input JSON file.
#[derive(Serialize, Deserialize)]
pub struct InputJson {
    dimensions: [u32; 3],
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

    let particle = ParticleBuilder::new().set_mass(input_json.mass).build();
}
