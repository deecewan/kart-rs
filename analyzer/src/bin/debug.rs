use clap::{Parser};
use std::path::PathBuf;

/// Run test analyses on a bunch of screenshots
#[derive(Parser, Debug)]
struct Args {

    /// A list of images, screenshots of races, to process to get the race update
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

      let result = analyzer::analyze(&image);
      let printable_res = match result {
          Some(screen) => format!("{:?}", screen),
          _ => "Unknown".into(),
      };
      println!("Result: {}", printable_res);
  });
}
