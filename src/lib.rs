extern crate image;

use std::path::Path;

use image::DynamicImage;

pub fn get_image_from_location(location: &str) -> DynamicImage {
    let input_path = Path::new(&location);
    image::open(&input_path).unwrap()
}
