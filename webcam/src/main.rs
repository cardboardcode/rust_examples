use anyhow::Result;
use opencv::{
    prelude::*,
    videoio,
    highgui,
    imgproc,
};

fn main() -> Result<()> {
    highgui::named_window("window", highgui::WINDOW_NORMAL)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    let mut resized_frame = Mat::default();

    loop {
        cam.read(&mut frame)?;
        
        // Get the original frame dimensions
        let original_height = frame.rows() as f64;
        let original_width = frame.cols() as f64;
        
        // Calculate the new width while maintaining the aspect ratio
        let new_height = 200.0;
        let new_width = (original_width / original_height * new_height) as i32;
        
        // Resize the frame
        imgproc::resize(
            &frame,
            &mut resized_frame,
            opencv::core::Size::new(new_width, 200),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )?;

        highgui::imshow("window", &resized_frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // quit with q
            break;
        }
    }
    Ok(())
}