use kart_rs::util::*;
use std::time::Duration;

fn main() {
    let mut count = 0;
    let mut total_time: Duration = Duration::from_secs(0);
    std::fs::read_dir("/Users/david/projects/ferocia/kartalytics/analyser/training-data")
        .expect("couldn't open dir")
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_file())
        .map(|entry| entry.path())
        .for_each(|path| {
            println!("path: {:?}", path);
            let image = image::open(&path).expect("failed to open static image");
            print_dynamic_image(path.to_str().unwrap(), &image);

            let time = timed_frame(&image);

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
