use base64::prelude::*;
use std::io::{Cursor, Read, Seek};
use std::path::PathBuf;
use std::time::Duration;

pub fn timed_frame(image: &image::DynamicImage) -> Duration {
    let start = std::time::Instant::now();
    let result = super::frame_process::process(image.clone());
    let end = std::time::Instant::now();

    match result {
        Some(crate::screens::Screen::Race(race)) => {
            println!("result: Race Screen\n{}", race);
        }
        _ => {
            println!("result: {:?}", result);
        }
    }

    let delta = end - start;

    let fps = Duration::from_secs(1).as_micros() / delta.as_micros();
    println!("∆ {:?}. fps: {:?}", delta, fps);

    delta
}

pub fn print_image_from_path(path: &PathBuf) {
    println!("image path: {:?}", &path);
    let name = path.to_str().unwrap();
    let bytes = std::fs::read(&path).unwrap();
    print_image(name, &bytes, 384, 216);
}

pub fn print_dynamic_image(name: &str, image: &image::DynamicImage) {
    let mut c = Cursor::new(Vec::new());
    image
        .write_to(&mut c, image::ImageOutputFormat::Jpeg(75))
        .expect("Couldn't get the image to print");
    c.rewind().expect("Couldn't get the image to print");
    let bytes: Vec<u8> = c.bytes().map(|b| b.unwrap()).collect();
    let width = image.width().min(384);
    let height = image.height().min(216);
    print_image(name, &bytes, width, height);
}

pub fn print_image(name: &str, bytes: &[u8], width: u32, height: u32) {
    let file_name = BASE64_STANDARD.encode(name);
    let file_contents = BASE64_STANDARD.encode(bytes);
    println!(
        "\x1b]1337;File=name={file_name};inline=1;width={width}px;height={height}px:{file_contents}\x07",
    );
}
