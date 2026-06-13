use core::error;
use std::{env, fs};

mod particle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // As a basic test of functionality, copy the input file into the output
    // directory.
    let file_contents = fs::read_to_string(&args[1])?;
    fs::write(
        "output/".to_owned() + &args[1].split("/").collect::<Vec<&str>>()[1],
        &file_contents,
    )?;
    println!("{}", file_contents);

    // Read file contents as a CSV.
    let mut file_args = file_contents.split("\n");

    // Parse the dimensions.
    let mut dimension_args = match file_args.next() {
        None => panic!("File is empty."),
        Some(line) => line.split(", "),
    };
    let width = match dimension_args.next() {
        None => panic!("Dimension arguments do not have enough values."),
        Some(width_arg) => width_arg.parse::<i32>(),
    };
    let length = match dimension_args.next() {
        None => panic!("Dimension arguments do not have enough values."),
        Some(length_arg) => length_arg.parse::<i32>(),
    };
    let height = match dimension_args.next() {
        None => panic!("Dimension arguments do not have enough values."),
        Some(height_arg) => height_arg.parse::<i32>(),
    };

    // Parse the mass.
    let mass = match file_args.next() {
        None => panic!("No mass provided"),
        Some(mass_arg) => mass_arg.parse::<i32>(),
    };

    Ok(())
}
