use crate::emit::EmitMode;

use super::emit::Emit;
use chrono::Utc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use uvc::Device;

pub fn from_device(device: &Device, store_frames: bool) {
    let handle = device.open().expect("couldn't open handle to device");

    let emitter = Emit::new(EmitMode::Real);

    let format = uvc::StreamFormat {
        width: 1920,
        height: 1080,
        fps: 30,
        format: uvc::FrameFormat::Any,
    };

    let mut stream_handle = handle
        .get_stream_handle_with_format(format)
        .expect("Could not open a stream with this format");

    let frame_folder_name = if store_frames {
        let name = format!("{}", Utc::now().to_rfc3339());

        std::fs::create_dir(format!("frames/{}", name))
            .expect("unable to create frame saving directory");
        Some(name)
    } else {
        None
    };

    let counter = Arc::new(AtomicUsize::new(0));
    // Get a stream, calling the closure as callback for every frame
    let _stream = stream_handle
        .start_stream(
            move |frame, count| {
                let start = std::time::Instant::now();
                count.fetch_add(1, Ordering::SeqCst);
                let bytes = frame.to_bytes();

                if bytes.len() < 20_000 {
                    // ignore
                    return;
                }

                let res = match image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg)
                {
                    Ok(frame) => {
                        if let Some(name) = &frame_folder_name {
                            let output_path = format!("frames/{}/frame_{:?}.jpg", name, count);
                            if let Err(e) = frame.save(&output_path) {
                                eprintln!("Failed to save image `{}`: {:?}", output_path, e);
                            }
                        }

                        crate::frame_process::process(frame)
                    }
                    Err(e) => {
                        eprintln!("Err! {:?}", e);
                        None
                    }
                };

                emitter.emit(&res);

                let end = std::time::Instant::now();

                let delta = end - start;

                let fps = Duration::from_secs(1).as_micros() / delta.as_micros();
                let printable_res = match res {
                    Some(screen) => match screen {
                        crate::screens::Screen::Race(race) => serde_json::to_string(&race).unwrap(),
                        _ => format!("{:?}", screen),
                    },
                    _ => "Unknown".into(),
                };
                println!("{} (fps: {:?})", printable_res, fps);
            },
            counter.clone(),
        )
        .expect("Could not start stream");

    loop {}
}
