use clap::Parser;
use rayon::prelude::*;
use std::{ffi::OsStr, path::PathBuf};

/// Will generate images at the size they get processed at. Specifically, (for
/// now), 1280x720
#[derive(Parser, Debug)]
struct Args {
    /// A list of images to downscale. It is assumed that these images will be
    /// 1920x1080.
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    args.files.par_iter().for_each(|file| {
        let image = match image::open(file) {
            Ok(image) => image,
            Err(e) => {
                eprintln!("ERROR: failed to open {file:?}: {e}");
                return;
            }
        };
        let (parent, stem) = match (file.parent(), file.file_stem().and_then(OsStr::to_str)) {
            (Some(parent), Some(stem)) => (parent, stem),
            _ => {
                eprintln!("ERROR: failed to extract file location for {file:?}");
                return;
            }
        };

        let output_location = parent.join(format!("{stem}_resized.jpg"));

        if let Err(e) = image
            .resize(1280, 720, image::imageops::Nearest)
            .save(&output_location)
        {
            eprintln!("ERROR: failed to save resized image for {file:?}: {e}");
            return;
        }

        println!("Downsized {file:?} to {output_location:?}");
    })
}
