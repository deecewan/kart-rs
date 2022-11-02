use kart_rs::util::*;
use std::path::Path;

fn main() {
    let image = "/Users/david/projects/ferocia/kartalytics/analyser/training-data/2017062119574100-16851BE00BC6068871FE49D98876D6C5.jpg";

    let path = Path::new(image);

    print_image_from_path(&path.to_path_buf());

    let image = image::open(path).expect("failed to open static image");
    timed_frame(&image);
}
