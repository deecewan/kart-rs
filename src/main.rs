use analyzer::{analyze, Screen};
use clap::Parser;
use stream;

use log::{error, info};
use log_err::LogErrResult;
use simplelog;
use std::fs::File;

mod cli;

fn main() {
    let args = cli::Cli::parse();
    init_logger(&args);

    let frame_saver = FrameSaver::new(args.store_frames);

    let emitter = emitter::Emit::new(emitter::Mode::Real);

    stream::device::from_device(move |frame, count| {
        frame_saver.save(frame, count);
        let start = std::time::Instant::now();

        let res = analyze(frame);

        if let Some(res) = &res {
            // we want to not emit "unknown screen" events
            if res != &Screen::Unknown {
                emitter.emit(res.event_type(), res);
            }
        }

        let end = std::time::Instant::now();
        let delta = end - start;
        let fps = std::time::Duration::from_secs(1).as_micros() / delta.as_micros();

        let output = match &res {
            None => "Unknown".into(),
            Some(screen) => {
                let json = serde_json::to_string(&screen);
                format!("{}: {:?}", screen.event_type(), json)
            }
        };

        info!("{output} ({fps} fps)");
    });
}

struct FrameSaver {
    name: Option<String>,
}

impl FrameSaver {
    fn new(save: bool) -> Self {
        let name = if save {
            let name = format!("{}", chrono::Utc::now().to_rfc3339());
            std::fs::create_dir_all(format!("frames/{}", name))
                .log_expect("unable to create frame saving directory");

            Some(name)
        } else {
            None
        };

        FrameSaver { name }
    }

    fn save(&self, frame: &image::DynamicImage, count: usize) {
        let Some(name) = &self.name else { return; };

        let output_path = format!("frames/{}/frame_{:?}.jpg", name, count);
        if let Err(e) = frame.save(&output_path) {
            error!("Failed to save frame: {:?}", e);
        }
    }
}

fn init_logger(args: &cli::Cli) {
    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![];

    let level = match &args.log {
        cli::LogLevel::Debug => simplelog::LevelFilter::Debug,
        cli::LogLevel::Info => simplelog::LevelFilter::Info,
        cli::LogLevel::Error => simplelog::LevelFilter::Error,
    };

    loggers.push(simplelog::TermLogger::new(
        level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    ));

    if args.store_logs {
        let file = File::options()
            .append(true)
            .truncate(false)
            .open("kart.log")
            .log_expect("couldn't open kart.log for logging");

        loggers.push(simplelog::WriteLogger::new(
            level,
            simplelog::Config::default(),
            file,
        ));
    }

    simplelog::CombinedLogger::init(loggers).log_expect("Couldn't initialize the logging service");
}
