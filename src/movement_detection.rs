pub fn calculate_diff_pixelwise(image1: &Vec<f32>, image2: &Vec<f32>) -> Vec<f32> {
    if image1.len() != image2.len() {
        panic!("Image1 size {} did not match image2 size {}", image1.len(), image2.len());
    }
    let mut diff_vec: Vec<f32> = vec![];
    for n in 0..image1.len()-1 {
        let diff = image1[n] - image2[n];
        diff_vec.push(diff.abs());
    }
    return diff_vec;
}
