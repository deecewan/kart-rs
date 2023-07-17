use analyzer::intro::Intro;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, ValueEnum, Clone)]
enum ToGenerate {
    Variant,
    Track,
}

impl ToGenerate {
    fn to_str(&self) -> &str {
    match self {
        ToGenerate::Variant => "variant",
        ToGenerate::Track => "track",
    }
    }
}

/// Generate new intro reference images, either for new or existing tracks. By
/// default, this will output new reference images next to the input image, with
/// a suffix of `_track.png` for track references, and `_variant.png` for
/// variant references
#[derive(Parser, Debug)]
struct Args {
    /// What reference to generate. The default is just to generate the `track`
    /// reference, because most of the variant images will be stable/not added
    /// to. Pass this argument multiple times to generate multiple values
    #[arg(long, value_enum, default_values_t = [ToGenerate::Track])]
    generate: Vec<ToGenerate>,

    /// A list of images, screenshots of track intros, to process to get the
    /// track reference, variant reference, or both, from.
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("args: {args:?}");

    args.files.iter().for_each(|path| {
        let image = match image::open(path) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("ERROR: Couldn't open image at {path:?} - {e}");
                return;
            }
        };

        if let Some(analyzer::Screen::Intro(Intro { course })) = analyzer::analyze(&image) {
            println!("Image at {path:?} resolved to a known screen - '{course}'");
        }

        args.generate.iter().for_each(|t| process(path, &image, &t));
    });
}

fn process(path: &PathBuf, image: &image::DynamicImage, generate: &ToGenerate) {
    let reference = match generate {
        ToGenerate::Track => analyzer::intro::get_track_image(image),
        ToGenerate::Variant => analyzer::intro::get_variant_image(image),
    };

    let Some(output_location) = get_output_location(path, generate.to_str()) else {
        eprintln!("Couldn't locate the correct place to save the new {} reference. Screen path: {path:?}", generate.to_str());
        return;
    };

    if let Err(e) = reference.save(output_location) {
        eprintln!("Failed to save new {} reference - {e}", generate.to_str());
    }

}

fn get_output_location(path: &PathBuf, generate_str: &str) -> Option<PathBuf> {
    let (parent, stem) = match (path.parent(), path.file_stem().and_then(|s| s.to_str())) {
        (Some(parent), Some(stem)) => (parent, stem),
        _ => {
            return None;
        }
    };
    let file_name = format!("{stem}_{generate_str}.png");

    return Some(parent.join(file_name));
}
