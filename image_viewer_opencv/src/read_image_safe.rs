use opencv::{
    core::Vec3b, // Import TypedMat
    imgcodecs,
    prelude::*,
};

fn main() -> Result<(), opencv::Error> {
    println!("[Running test script...]");

    let image_path = "test.png";

    // Read the image using OpenCV
    let image = imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR)?;

    // Check if the image was loaded successfully
    if image.empty() {
        println!("Could not open or find the image: {}", image_path);
        return Ok(());
    }

    // Get the image dimensions (rows, columns, channels)
    let rows = image.rows() as usize;
    let cols = image.cols() as usize;
    let channels = image.channels() as usize;

    // Create an empty vector to store the 1D array
    // let mut one_d_vector: Vec<u8> = Vec::new();

    println!("rows = {}", rows);
    println!("cols = {}", cols);
    println!("channels = {}", channels);

    // Create an empty vector to store the 1D array
    let mut one_d_vector: Vec<u8> = Vec::new();

    // Iterate through each pixel and push its values into the vector
    for i in 0..rows {
        for j in 0..cols {
            for _k in 0..channels {
                let pixel = image.at_2d::<Vec3b>(i as i32, j as i32)?;
                let pixel_array = *pixel;
                one_d_vector.push(pixel_array[0]);
                one_d_vector.push(pixel_array[1]);
                one_d_vector.push(pixel_array[2]);
            }
        }
    }

    println!("Image dimensions: {}x{} with {} channels", rows, cols, channels);
    println!("1D vector length: {}", one_d_vector.len());


    Ok(())

}
