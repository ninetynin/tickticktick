#![allow(warnings)]

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::env;
use std::path::Path;
use directories::{/*BaseDirs, UserDirs,*/ ProjectDirs};
use chrono::Local;

use std::rc::Rc;
use std::fs;


pub fn App() -> ! {
    // let curr_wall: String = wallpaper::get().unwrap();
    let mut path = Path::new("F:\\GITHUB REPOS\\ninetynin\\tickticktick\\assets\\sample.png");

    let (width, height) = (1920.00, 1080.00);
    let mut image = RgbImage::new(width as u32, height as u32);
    for pixel in image.pixels_mut() {
        *pixel = Rgb([0u8, 0u8, 0u8]);
    } // black background

    // let mut black_bg_img_path = Path::new("../assets/black_bg_dont_modify.png");

    // let mut image = image::open("F:\\GITHUB REPOS\\ninetynin\\tickticktick\\assets\\black_bg_dont_modify.png").unwrap().to_rgb8();

    // let mut i2 = image.clone();

    let font = Font::try_from_vec
        (Vec::from(include_bytes!("../fonts/CrimsonPro-VariableFont_wght.ttf") as &[u8])).unwrap();

    let scale = Scale::uniform(250.0);

    loop {
        for pixel in image.pixels_mut() {
            *pixel = Rgb([0u8, 0u8, 0u8]);
        }
        let (x1,y1) = (width/3.2, height/3.2);
        draw_text_mut(
            &mut image,
            Rgb([255u8, 255u8, 255u8]),
            x1 as i32, 
            y1 as i32, 
            scale,
            &font,
            Local::now().format("%H:%M:%S").to_string().as_str(),
        );
        let _ = image.save(&path).unwrap();
        wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
        // fs::remove_file(path).unwrap();
    }
    // !TODO implement clap crate so that tickticktick quit quits the program
}

pub fn path_setup() -> String {
    // let pdir = ProjectDirs::from("com", "tickticktick", "images").unwrap();
    if let Some(pdir) = ProjectDirs::from("com", "", "tickticktick") {
        let mut path = pdir.data_dir().to_str().unwrap().to_string();
        return path.to_string();
    } else {
        panic!("Could not find project directory");
    }
    // let mut path = pdir.data_dir().to_str().unwrap().to_string();
    // path.to_string()
}

mod tests {
    #[test]
    fn test_path_setup() {
        dbg!(super::path_setup());
    }
}