use image::{ GenericImageView, ImageBuffer, Luma, GrayImage };

pub fn read_image(_path: String) -> Vec<u8> {
    let img = image::open(_path).expect("File not found!");
    let (width, height) = img.dimensions();
    let mut pixel_values: Vec<u8>  = vec![0; usize::try_from(width).unwrap() * usize::try_from(height).unwrap() * 5];
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
    let (width, height) = img.dimensions();
    let mut pixel_values: Vec<f32>  = vec![0.0; usize::try_from(width).unwrap() * usize::try_from(height).unwrap()];
    image_grayscale.iter().map(|it| it).for_each( |it : &u8|
        pixel_values.push((*it as f32) / 255_f32)
    );
    return pixel_values;
}

pub fn read_image_dimensions(_path: String) -> (u32, u32) {
    return image::open(_path).expect("File not found!").dimensions();
}

pub fn create_grayscale_image(_image_vec: &Vec<f32>, _width: u32, _height: u32) {
    let mut image: GrayImage = ImageBuffer::new(_width, _height);
    for width_index in 0.._width-1 {
        for height_index in 0.._height-1 {
            let idx = (width_index*height_index+height_index) as usize;
            *image.get_pixel_mut(width_index, height_index) = image::Luma([(_image_vec[idx] * 255_f32) as u8]);
        }
    }
    image.save("output.png").unwrap();
}