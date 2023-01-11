use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use kart_rs::reference::Reference;
use kart_rs::screens::intro::INTRO_HASHES;
use kart_rs::util::*;

fn main() {
    std::fs::create_dir("intro_references").unwrap_or(());
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

        let res = kart_rs::screens::intro::Intro::process(&image);

        match res {
            Some(r) => println!("Looks like a {:?}", r),
            None => println!("Doesn't look like one we have..."),
        }

        let name = show_selector();
        println!("Name: {:?}", name);
    });
}

fn show_selector() -> Option<&'static str> {
    let items: Vec<&str> = INTRO_HASHES.iter().map(|r| r.name).collect();
    let selected = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the correct track:")
        .items(&items)
        .max_length(5)
        .interact();

    match selected {
        Ok(i) => INTRO_HASHES.get(i).map(|r| r.name),
        Err(e) => panic!("Error selecting a track: {:?}", e),
    }
}
