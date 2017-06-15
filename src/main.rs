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
        .get_matches();

    let levels = 3;
    let input_path = matches.value_of("input").unwrap();

    let res_0 = image::open(&Path::new(&input_path)).unwrap();
    let mut pyramid = vec![res_0];
    for level in 1..levels {
        let down_sampled_image = {
            let ref higher_res_image = pyramid[level - 1];
            let (width, height) = higher_res_image.dimensions();

            higher_res_image.resize(width / 4, height / 4, FilterType::Gaussian)
        };

        pyramid.push(down_sampled_image);
    }

    let pyramid_path = "./mwahaha";
    DirBuilder::new()
        .recursive(true)
        .create(pyramid_path)
        .unwrap();

    for image in &pyramid {
        let width = image.width();
        let file_name = format!("image-{}x{}", width, width);
        let out_path = Path::new(pyramid_path).join(file_name);
        let ref mut fout = File::create(&out_path).unwrap();
        let _ = image.save(fout, image::PNG).unwrap();
    }
}
