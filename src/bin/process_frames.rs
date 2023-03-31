use analyzer;
use clap::Parser;
use emitter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let emitter = emitter::Emit::<analyzer::Screen>::new(emitter::Mode::Real);

    args.files.iter().for_each(|path| {
        if path.is_dir() {
            process_dir(path, &emitter);
        } else {
            process(path, &emitter);
        }
    });

    loop {}
}

fn process(path: &PathBuf, emitter: &emitter::Emit<analyzer::Screen>) {
    println!("Processing {:?}", path);
    let image = image::open(&path)
        .expect("failed to open static image")
        .resize(1280, 720, image::imageops::Nearest);

    let result = analyzer::analyze(&image);

    if let Some(ref result) = result {
        emitter.emit(result.event_type(), &result);
    }

    let printable_res = match result {
        Some(screen) => format!("{:?}", screen),
        _ => "Unknown".into(),
    };

    println!("Result: {}", printable_res);
}

fn process_dir(path: &PathBuf, emitter: &emitter::Emit<analyzer::Screen>) {
    let mut paths = std::fs::read_dir(path)
        .expect("couldn't open dir")
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap().path())
        .filter(|f| f.is_file())
        .collect::<Vec<PathBuf>>();

    paths.sort_unstable_by(|a, b| {
        let first = String::from(
            a.file_stem()
                .unwrap()
                .to_str()
                .expect("couldn't make the file name a string"),
        );
        let (_, first) = first.rsplit_once('_').expect("failed to split first");
        let second = String::from(
            b.file_stem()
                .unwrap()
                .to_str()
                .expect("couldn't make the file name a string"),
        );
        let (_, second) = second.rsplit_once('_').expect("failed to split second");

        let first = first.parse::<usize>().expect("couldn't parse the number");
        let second = second.parse::<usize>().expect("couldn't parse the number");

        return first.cmp(&second);
    });

    paths.iter().for_each(|f| process(&f, emitter));
}
