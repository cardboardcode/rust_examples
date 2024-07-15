use opencv::{
    highgui,
    prelude::*,
    videoio,
};
use std::time::Instant;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Camera
    let setup_start = Instant::now();
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;

    // If Camera is still unopened, exit program.
    if !opened {
        println!("Unable to open default camera!");
        return Ok(());
    }
    // Set frame height and width
    cam.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?; 
    cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
    let setup_duration = setup_start.elapsed().as_secs_f64(); // Calculate setup duration
    println!("Setup duration: {:?}", setup_duration); // Print setup duration

    // Measure FPS
    let fps_start = Instant::now();
    let num_frames_to_measure = 60;
    for _ in 0..num_frames_to_measure {
        let mut temp_frame = Mat::default();
        cam.read(&mut temp_frame)?;
    }
    let fps_duration = fps_start.elapsed();
    let fps = num_frames_to_measure as f64 / fps_duration.as_secs_f64();
    println!("Measured FPS: {:?}", fps);

    // Display video stream from Camera
    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow("camera", &frame)?;
            // Press 'q' program to exit
            let key = highgui::wait_key(1)?;
            if key == 13 {  
                break;
            }
        }
    }

    Ok(())
}
