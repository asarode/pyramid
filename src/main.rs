extern crate clap;
extern crate image;

use std::fs::{DirBuilder, File};
use std::path::Path;

use clap::{Arg, App};
use image::{GenericImage, FilterType};

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
    let input_path = Path::new(&input_location);
    let image_name = input_path.file_stem().unwrap().to_str().unwrap();
    let extension = input_path.extension().unwrap();
    let inital_image = image::open(&input_path).unwrap();

    let mut pyramid = vec![inital_image];
    let levels = 3;
    for level in 1..levels {
        let down_sampled_image = {
            let ref higher_res_image = pyramid[level - 1];
            let (width, height) = higher_res_image.dimensions();

            higher_res_image.resize(width / 4, height / 4, FilterType::Gaussian)
        };

        pyramid.push(down_sampled_image);
    }

    DirBuilder::new()
        .recursive(true)
        .create(output_location)
        .unwrap();

    for image in &pyramid {
        let width = image.width();
        let file_name = format!("{}-{}x{}", image_name, width, width);
        let out_path = Path::new(output_location).join(file_name).with_extension(extension);
        let ref mut fout = File::create(&out_path).unwrap();
        let _ = image.save(fout, image::JPEG).unwrap();
    }
}
