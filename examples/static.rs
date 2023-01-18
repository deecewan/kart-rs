use kart_rs::util::*;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    pub path: PathBuf,
}

fn main() {
    let args = Opts::parse();

    print_image_from_path(&args.path);

    let image = image::open(&args.path).expect("failed to open static image");
    timed_frame(&image);
}
