use std::io;
use std::io::Write;
use opencv::{
    core,
    highgui,
    prelude::*,
    videoio,
};

fn run()  -> opencv::Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;

    //  Issue below:
    //  'from_file' by it self, and 'from_file' followed by 'open_file' both fail it letting me successfully
    // perform the subsequent 'read'
    let file_name = "test.mp4";    
    let mut cam = videoio::VideoCapture::from_file(&file_name, videoio::CAP_ANY)?;
    let opened_file = videoio::VideoCapture::open_file(&mut cam, &file_name, videoio::CAP_ANY)?;
    if !opened_file {
        panic!("Unable to open video file2!");
    };
    let mut frame = core::Mat::default();
    let frame_read = videoio::VideoCapture::read(&mut cam, &mut frame)?; 
    if !frame_read {
        panic!("Unable to read from video file!");
    };
    let opened = videoio::VideoCapture::is_opened(&mut cam)?;
    println!("Opened? {}", opened);
    if !opened {
        panic!("Unable to open video file!");
    };
    //  Issue above:**_
    loop {

        videoio::VideoCapture::read(&mut cam, &mut frame)?; 
        if frame.size()?.width > 0 {
            println!("[INFO] - Displaying frame...");
            io::stdout().flush().unwrap();
            highgui::imshow(window, &frame)?;
            let key = highgui::wait_key(41)?;
            if key > 0 && key != 255 {
                videoio::VideoCapture::release(&mut cam)?;
                break;
            }
        }
        else {
            println!("No more frames!");
            videoio::VideoCapture::release(&mut cam)?;
            break();
        }
    }
    Ok(())

}

fn main() {
    run().unwrap()
}