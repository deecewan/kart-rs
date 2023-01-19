use clap::Parser;
use kart_rs::util::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    args.files.iter().for_each(|path| {
        println!("Processing {:?}", path);
        let image = image::open(&path).expect("failed to open static image");
        print_dynamic_image(path.to_str().unwrap(), &image);

        let result = kart_rs::frame_process::process(image.clone());

        println!("Result: {:?}", result);
    });
}
