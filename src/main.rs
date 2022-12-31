use crate::movement_detection::{calculate_diff, calculate_diff_pixelwise};

mod image_utils;
mod movement_detection;

fn main() {
    println!("Reading image from memory");
    let pixel_vec_image1: Vec<f32> = image_utils::read_image_grayscale(String::from("test.png"));
    let pixel_vec_image2: Vec<f32> = image_utils::read_image_grayscale(String::from("test2.png"));
    let pixel_diff_vec = calculate_diff_pixelwise(&pixel_vec_image1, &pixel_vec_image2);
    for pixel in pixel_diff_vec {
        println!("diff: {}", pixel);
    }
}
