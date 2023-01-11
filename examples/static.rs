use kart_rs::util::*;
use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[arg(long)]
    pub path: String,
}

fn main() {
    let args = Opts::parse();
    let path = Path::new(&args.path);

    print_image_from_path(&path.to_path_buf());

    let image = image::open(path).expect("failed to open static image");
    timed_frame(&image);
}
