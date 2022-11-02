use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use uvc::Device;

const STORE_FRAMES: bool = false;

pub fn from_device(device: &Device) {
    let handle = device.open().expect("couldn't open handle to device");

    let format = uvc::StreamFormat {
        width: 1920,
        height: 1080,
        fps: 30,
        format: uvc::FrameFormat::Any,
    };

    let mut stream_handle = handle
        .get_stream_handle_with_format(format)
        .expect("Could not open a stream with this format");

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
                        if STORE_FRAMES {
                            if let Err(e) = frame.save(format!("frame_{:?}.jpg", count)) {
                                println!("Failed to save image `frame_{:?}.jpg`: {:?}", count, e);
                            }
                        }

                        Some(crate::frame_process::process(frame))
                    }
                    Err(e) => {
                        eprintln!("Err! {:?}", e);
                        None
                    }
                };

                let end = std::time::Instant::now();

                let delta = end - start;

                let fps = Duration::from_secs(1).as_micros() / delta.as_micros();
                let printable_res = match res {
                    Some(Some(screen)) => format!("{:?}", screen),
                    _ => "Unknown".into(),
                };
                println!("{} (fps: {:?})", printable_res, fps);
            },
            counter.clone(),
        )
        .expect("Could not start stream");

    loop {}
}
