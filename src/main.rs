use crate::movement_detection::{calculate_diff_pixelwise};

mod image_utils;
mod movement_detection;

fn main() {
    println!("Reading image from file");
    let pixel_vec_image1: Vec<f32> = image_utils::read_image_grayscale(String::from("test.png"));
    let pixel_vec_image2: Vec<f32> = image_utils::read_image_grayscale(String::from("test2.png"));
    let pixel_diff_vec = calculate_diff_pixelwise(&pixel_vec_image1, &pixel_vec_image2);
    /*for pixel in pixel_vec_image2 {
        println!("diff: {}", pixel);
    }
    */
    let dims: (u32, u32) = image_utils::read_image_dimensions(String::from("test.png"));
    image_utils::create_grayscale_image(&pixel_vec_image2, dims.0, dims.1)
}
