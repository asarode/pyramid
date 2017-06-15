extern crate clap;
extern crate image;
extern crate pyramid_grok;

use std::fs::DirBuilder;

use clap::{Arg, App};
use pyramid_grok::{build_pyramid, get_image_from_location, write_image_tile};

fn main() {
    let matches = App::new("Pyramid")
        .version("1.0")
        .author("Arjun Sarode <sarodearjun57@gmail.com>")
        .about("Outputs an image pyramid from an input image.")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("IMAGE_FILE")
            .help("Sets the input file to use")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("OUT_DIR")
            .help("Sets the output directory where transformed images will be written")
            .takes_value(true))
        .get_matches();

    let input_location = matches.value_of("input").unwrap();
    let output_location = matches.value_of("output").unwrap_or("./mwahaha");
    let initial_image = get_image_from_location(input_location);

    let levels = 3;
    let pyramid = build_pyramid(levels, initial_image);

    let out_dir = DirBuilder::new()
        .recursive(true)
        .create(output_location);
    match out_dir {
        Ok(_) => (),
        Err(error) => panic!("There was a problem using the output directory {}: {:?}", output_location, error),
    }

    println!("Outputting images to: {}", output_location);
    for image in &pyramid {
        write_image_tile(input_location, output_location, image);
    }
}
