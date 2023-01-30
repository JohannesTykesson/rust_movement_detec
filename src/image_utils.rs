use image::{ GenericImageView, ImageBuffer, Luma, GrayImage };

pub fn read_image(_path: String) -> Vec<u8> {
    let img = image::open(_path).expect("File not found!");
    let mut pixel_values: Vec<u8> = Vec::new();
    for pixel in img.pixels() {
        let (_x, _y, _rgba) = pixel; 
        pixel_values.push(_rgba.0[0]); // R
        pixel_values.push(_rgba.0[1]); // G
        pixel_values.push(_rgba.0[2]); // B
        pixel_values.push(_rgba.0[3]); // Alpha
    }
    return pixel_values;
}

pub fn read_image_grayscale(_path: String) -> Vec<f32> {
    let img = image::open(_path).expect("File not found!");
    let image_grayscale: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    let mut pixel_values: Vec<f32> = Vec::new();
    image_grayscale.iter().for_each(|f|
        pixel_values.push((*f as f32) / 255_f32)
    );
    return pixel_values;
}

pub fn read_image_dimensions(_path: String) -> (u32, u32) {
    return image::open(_path).expect("File not found!").dimensions();
}

pub fn create_grayscale_image(_image_vec: &Vec<f32>, _width: u32, _height: u32) {
    println!("Creating img with dims w*h {}*{}", _width, _height);
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut image: GrayImage = ImageBuffer::new(_width, _height);
    for idx in 0.._image_vec.len()-1 {
        *image.get_pixel_mut(x, y) = image::Luma([(_image_vec[idx] * 255_f32) as u8]);
        x += 1;
        if x == _width {
            x = 0;
            y += 1;
        }
    }
    image.save("output.png").unwrap();
}