use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::env;
use std::path::Path;
use wallpaper;
use directories::{/*BaseDirs, UserDirs,*/ ProjectDirs};
// use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;

fn main() {
    // let pdir = ProjectDirs::from("com", "tickticktick", "images").unwrap();
    // let path = pdir.data_dir().join("wallpaper.png");
    // BEST USE CASE _> BLACK BACKGROUND
    // let curr_wall: String = wallpaper::get().unwrap();
    let path = Path::new("F:\\GITHUB REPOS\\ninetynin\\tickticktick\\assets\\sample.png");

    let mut image = RgbImage::new(1920, 1080);
    for pixel in image.pixels_mut() {
        *pixel = Rgb([0u8, 0u8, 0u8]);
    } // black background

    // let font = Vec::from(include_bytes!("../fonts/DejaVuSans.ttf") as &[u8]);
    let font = Vec::from(include_bytes!("../fonts/CrimsonPro-VariableFont_wght.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // let height = 28.4;
    // let scale = Scale {
    //     x: height * 2.0,
    //     y: height * 1.5,
    // };
    let scale = Scale::uniform(100.0);

    // let text = "Hello, world!";
    loop {
        for pixel in image.pixels_mut() {
            *pixel = Rgb([0u8, 0u8, 0u8]);
        } // black background    
        let local = OffsetDateTime::now_local();
        let text = local.unwrap().to_string();
        draw_text_mut(
            &mut image,
            Rgb([255u8, 255u8, 255u8]),
            600,
            400,
            scale,
            &font,
            text.as_str(),
        );
        let _ = image.save(path.clone()).unwrap();
        wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
    }
    // let (w, h) = text_size(scale, &font, text);
    // println!("Text size: {}x{}", w, h);

    // let i = image.save(path).unwrap();
    // wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
    // println!("{:?}", wallpaper::get());
    // // wallpaper::set_from_url("https://source.unsplash.com/random").unwrap();
    // wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
}