extern crate clap;
extern crate image;

use std::fs::File;
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

    let input_file = matches.value_of("input").unwrap();
    let im = image::open(&Path::new(&input_file)).unwrap();
    let (width, height) = im.dimensions();
    let down_sampled = im.resize(width / 4, height / 4, FilterType::Gaussian);
    let ref mut fout = File::create(&Path::new("down-sampled.jpg")).unwrap();
    let _ = down_sampled.save(fout, image::PNG).unwrap();
}
