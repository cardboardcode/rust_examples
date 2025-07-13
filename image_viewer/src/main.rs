use fltk::{app, frame::Frame, image::SharedImage, prelude::*, window::Window, enums::*};
use std::error::Error;


pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

fn quit_cb() {
    app::quit();
}

fn menu_cb(m: &mut impl MenuExt) {
    if let Ok(mpath) = m.item_pathname(None) {
        match mpath.as_str() {
            "&File/&Quit\t" => quit_cb(),
            _ => unreachable!(),
        }
    }
}

// Initialise menu by adding key event.
fn init_menu(m: &mut fltk::menu::SysMenuBar) {
    
    // Add event capture in menu: Ctrl + q
    let idx = m.add(
        "&File/&Quit\t",
        Shortcut::Ctrl | 'q',
        fltk::menu::MenuFlag::Normal,
        menu_cb,
    );
    m.at(idx).unwrap().set_label_color(Color::Red);
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    
    let mut wind = Window::default()
        .with_size(400, 300)
        .with_pos(0, 0)
        .with_label("Image Viewer");

    let mut m = fltk::menu::SysMenuBar::default();
    init_menu(&mut m);

    // let pos_x= wind.x();
    // println!("pos_x = {pos_x:?}");

    let mut frame = Frame::default_fill();
    let mntr1 = app::Screen::new(0).expect("Could not find screen.");
    let width = mntr1.w();
    let height = mntr1.h();
    
    println!("Width =  {}", width);
    println!("Height =  {}", height);
    
    let mut image = SharedImage::load("test.png")?;
    image.scale(width, height, true, true);
    let image_height = image.height();
    let image_width = image.width();
    wind.set_size(image_width, image_height);

    frame.set_image(Some(image));

    // // To remove an image
    // frame.set_image(None::<SharedImage>);
    wind.end();
    wind.make_resizable(true);
    wind.show();

    app.run()?;
    Ok(())
}