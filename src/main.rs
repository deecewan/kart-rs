use clap::Parser;
use stream::from_device;
use analyzer::analyze;

mod cli;

fn main() {
    let args = cli::Cli::parse();

    let emitter = emitter::Emit::new(emitter::Mode::Real);
    let frame_saver = FrameSaver::new(args.store_frames);

    from_device(move |frame, count| {
        frame_saver.save(frame, count);
        let start = std::time::Instant::now();

        let res = analyze(frame);

        if let Some(res) = &res {
            emitter.emit(res.event_type(), res);
        }

        let end = std::time::Instant::now();
        let delta = end - start;
        let fps = std::time::Duration::from_secs(1).as_micros() / delta.as_micros();

        let output = match res {
            None => "Unknown".into(),
            Some(screen) => {
                let json = serde_json::to_string(&screen);
                format!("{:?}", json)
            },
        };

        println!("{output} ({fps} fps)");
    });
}

struct FrameSaver { name: Option<String> }

impl FrameSaver {
    fn new(save: bool) -> Self {
        let name = if save {
            let name = format!("{}", chrono::Utc::now().to_rfc3339());
            std::fs::create_dir_all(format!("frames/{}", name))
                .expect("unable to create frame saving directory");

            Some(name)
        } else { None };

        FrameSaver { name }
    }

    fn save(&self, frame: &image::DynamicImage, count: usize) {
        let Some(name) = &self.name else { return; };

        let output_path = format!("frames/{}/frame_{:?}.jpg", name, count);
        if let Err(e) = frame.save(&output_path) {
            eprintln!("Failed to save frame: {:?}", e);
        }
    }
}
