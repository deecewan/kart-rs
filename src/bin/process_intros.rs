use clap::{Parser, ValueEnum};
use dialoguer::Confirm;
use kart_rs::{screens::intro::get_track_image, util::print_dynamic_image};
use lazy_static::lazy_static;
use std::{path::PathBuf, str::FromStr};

#[derive(Clone, ValueEnum, Debug, PartialEq)]
enum DuplicateStrategy {
    Skip,
    Overwrite,
    Ask,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = DuplicateStrategy::Ask)]
    duplicate_strategy: DuplicateStrategy,

    files: Option<Vec<PathBuf>>,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

fn main() {
    println!("processing all the intros");

    let output_folder =
        PathBuf::from_str("references/intro/tracks/").expect("Couldn't create the path");
    if !output_folder.try_exists().unwrap_or(false) {
        panic!("Couldn't find the output folder at {:?}", output_folder);
    }

    let files: Vec<PathBuf> = match &ARGS.files {
        Some(files) => files.to_vec(),
        None => std::fs::read_dir("./intro-screens")
            .expect("couldn't open dir")
            .map(|entry| entry.unwrap().path())
            .filter(|p| p.extension().map(|e| e != ".ds_store").unwrap_or(false))
            .collect(),
    };

    files.iter().for_each(|path| {
        println!("Processing: {:?}", path);
        process_screen(path, &output_folder);
    });
}

fn process_screen(path: &PathBuf, output_folder: &PathBuf) {
    let image = image::open(&path).expect("failed to open static image");
    let track = get_track_image(&image);

    let file = path
        .file_name()
        .map(|p| p.to_str())
        .flatten()
        .expect("Couldn't get the file name");

    let output_location = {
        let mut f = output_folder.clone();
        f.push(file);
        f
    };

    if output_location.is_file() {
        println!("That reference already exists.");

        println!("Before:");
        let before = image::open(&output_location).expect("Couldn't open before image");
        print_dynamic_image("before", &before);
        println!("After:");
        print_dynamic_image("after", &track);

        let overwrite = match ARGS.duplicate_strategy {
            DuplicateStrategy::Skip => {
                println!("Skipping...");
                return;
            }
            DuplicateStrategy::Ask => Confirm::new()
                .with_prompt("Do you want to overwrite?")
                .interact()
                .expect("Couldn't read the result of the prompt???"),
            DuplicateStrategy::Overwrite => true,
        };

        if overwrite {
            track
                .save(output_location)
                .expect("Failed to save the image");
        } else {
            println!("Skipping...")
        }
    } else {
        print_dynamic_image("track", &track);
        track
            .save(output_location)
            .expect("Failed to save the image");
    }
}
