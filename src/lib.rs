#![allow(warnings)]

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::env;
use std::path::Path;
use directories::{/*BaseDirs, UserDirs,*/ ProjectDirs};
use chrono::Local;

use std::rc::Rc;


pub fn App() -> ! {
    // let pdir = ProjectDirs::from("com", "tickticktick", "images").unwrap();
    // let path = pdir.data_dir().join("wallpaper.png");
    // BEST USE CASE _> BLACK BACKGROUND
    // let curr_wall: String = wallpaper::get().unwrap();
    let mut path = Path::new("F:\\GITHUB REPOS\\ninetynin\\tickticktick\\assets\\sample.png");

    let (width, height) = (1920, 1080);
    let mut image = RgbImage::new(width, height);
    for pixel in image.pixels_mut() {
        *pixel = Rgb([0u8, 0u8, 0u8]);
    } // black background

    let mut i2 = image.clone();

    let font = Font::try_from_vec
        (Vec::from(include_bytes!("../fonts/CrimsonPro-VariableFont_wght.ttf") as &[u8])).unwrap();

    let scale = Scale::uniform(250.0);

    loop {
        image = i2.clone();
        draw_text_mut(
            &mut image,
            Rgb([255u8, 255u8, 255u8]),
            400, 
            400, 
            scale,
            &font,
            // text.as_str(),
            Local::now().format("%H:%M:%S").to_string().as_str(),
        );
        let _ = image.save(&path).unwrap();
        wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
    }
    // !TODO implement clap crate so that tickticktick quit quits the program
}