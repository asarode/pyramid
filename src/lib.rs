extern crate image;

use std::path::Path;
use std::fs::File;

use image::{DynamicImage, FilterType, GenericImage};

pub fn get_image_from_location(location: &str) -> DynamicImage {
    let input_path = Path::new(&location);
    image::open(&input_path).unwrap()
}

pub fn build_pyramid(levels: usize, initial_image: image::DynamicImage) -> Vec<DynamicImage> {
    let mut pyramid = vec![initial_image];
    for level in 1..levels {
        let down_sampled_image = {
            let ref higher_res_image = pyramid[level - 1];
            let (width, height) = higher_res_image.dimensions();

            higher_res_image.resize(width / 4, height / 4, FilterType::Gaussian)
        };

        pyramid.push(down_sampled_image);
    }

    pyramid
}

pub fn write_image_tile(input_location: &str, output_location: &str, image_to_write: &image::DynamicImage) -> () {
    let input_path = Path::new(&input_location);
    let image_name = input_path.file_stem().unwrap().to_str().unwrap();
    let extension = input_path.extension().unwrap();
    let width = image_to_write.width();
    let file_name = &*format!("{}-{}x{}", image_name, width, width);
    let out_path = Path::new(output_location).join(file_name).with_extension(extension);
    let ref mut fout = File::create(&out_path)
        .expect(&format!("Failed to create the file {} in directory {}", file_name, output_location));
    let _ = image_to_write.save(fout, image::JPEG).expect(&format!("Failed to save image to {}", out_path.to_str().unwrap()));

    ()
}
