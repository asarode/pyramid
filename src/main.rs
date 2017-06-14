extern crate clap;
extern crate image;

use std::fs::File;
use std::path::Path;

use clap::{Arg, App};

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
    let blurred_im_1 = im.blur(0.1);
    let blurred_im_2 = im.blur(1.0);
    let blurred_im_3 = im.blur(4.0);
    let blurred_im_4 = im.blur(16.0);
    let ref mut fout_1 = File::create(&Path::new("blurred-1.jpg")).unwrap();
    let ref mut fout_2 = File::create(&Path::new("blurred-2.jpg")).unwrap();
    let ref mut fout_3 = File::create(&Path::new("blurred-3.jpg")).unwrap();
    let ref mut fout_4 = File::create(&Path::new("blurred-4.jpg")).unwrap();
    let _ = blurred_im_1.save(fout_1, image::PNG).unwrap();
    let _ = blurred_im_2.save(fout_2, image::PNG).unwrap();
    let _ = blurred_im_3.save(fout_3, image::PNG).unwrap();
    let _ = blurred_im_4.save(fout_4, image::PNG).unwrap();
}
