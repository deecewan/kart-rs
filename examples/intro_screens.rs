use kart_rs::reference::Reference;
use kart_rs::util::*;
use std::time::{Duration, Instant};

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[arg(long)]
    pub save_reference: bool,
}

fn main() {
    let args = Opts::parse();
    let mut count = 0;
    let mut total_time: Duration = Duration::from_secs(0);
    if args.save_reference {
        std::fs::create_dir("intro_references").unwrap_or(());
    }
    std::fs::read_dir(
        "/Users/david/projects/ferocia/kartalytics/analyser/training-data/introscreens",
    )
    .expect("couldn't open dir")
    .map(|entry| entry.unwrap().path())
    .for_each(|path| {
        let image = image::open(&path).expect("failed to open static image");
        print_dynamic_image(path.to_str().unwrap(), &image);
        let reference = kart_rs::screens::intro::get_comparison_image(&image);
        print_dynamic_image("reference", &reference);

        if args.save_reference {
            let name = path
                .file_name()
                .map(|f| f.to_str().unwrap())
                .expect("couldn't get name for file");

            reference
                .save(format!("intro_references/{}.jpg", name))
                .unwrap();
        }

        let start = Instant::now();
        let res = kart_rs::screens::intro::Intro::process(&image);
        let time = Instant::now() - start;

        println!("{:?}", res);

        total_time = total_time + time;
        count = count + 1;
        let average_time = total_time / count;
        let fps = Duration::from_secs(1).as_micros() / average_time.as_micros();
        println!(
            "Processed: {}. Avg time: {:?}. FPS: {}",
            count, average_time, fps
        );
    });
}
