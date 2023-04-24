use dialoguer::{theme::ColorfulTheme, Select};
use image::DynamicImage;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use uvc;

pub fn from_device<F>(on_frame: F)
where
    F: 'static + Send + Sync + Fn(&DynamicImage, usize),
{
    ensure_root();
    let ctx = uvc::Context::new().expect("Could not get context");
    let device = get_device(&ctx);
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
                count.fetch_add(1, Ordering::SeqCst);
                let bytes = frame.to_bytes();

                if bytes.len() < 20_000 {
                    // ignore
                    return;
                }

                match image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg) {
                    Ok(frame) => {
                        on_frame(&frame, count.load(Ordering::SeqCst));
                    }
                    Err(e) => {
                        eprintln!("Err! {:?}", e);
                    }
                };
            },
            counter.clone(),
        )
        .expect("Could not start stream");

    loop {}
}

fn get_device<'a>(ctx: &'a uvc::Context) -> uvc::Device<'a> {
    let devices: Vec<uvc::Device> = ctx.devices().expect("Couldn't load devices").collect();
    let options: Vec<String> = devices
        .iter()
        .map(|d| {
            let description = d.description().unwrap();

            format!(
                "{} {}: {:#06x} {:#06x}",
                description.manufacturer.unwrap_or("?".to_string()),
                description.product.unwrap_or("?".to_string()),
                description.vendor_id,
                description.product_id,
            )
        })
        .collect();

    if options.is_empty() {
        let string = console::style("Couldn't find any capture devices.").red();
        eprintln!("{string}");
        std::process::exit(1);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a device:")
        .items(&options)
        .default(0)
        .interact();

    match selection {
        Ok(s) => match devices.get(s) {
            Some(d) => {
                let desc = d.description().expect("couldn't get device description");

                ctx.find_device(
                    Some(desc.vendor_id as i32),
                    Some(desc.product_id as i32),
                    None,
                )
                .expect("Could not find device")
            }
            None => {
                panic!("Selected device not in device array.");
            }
        },
        Err(e) => {
            panic!("Error with device selection: {:?}", e);
        }
    }
}

fn ensure_root() {
    // TODO: maybe others? idk
    if cfg!(target_os = "macos") {
        if !nix::unistd::Uid::effective().is_root() {
            let string = format!(
                "|{}|
|{}|
|{}|
|{}|
|{}|
|{}|",
                console::pad_str_with("", 80, console::Alignment::Center, None, '-'),
                console::pad_str("", 80, console::Alignment::Center, None),
                console::pad_str(
                    "This program must be run as root in order to",
                    80,
                    console::Alignment::Center,
                    None
                ),
                console::pad_str(
                    "correctly stream the USB Camera",
                    80,
                    console::Alignment::Center,
                    None
                ),
                console::pad_str("", 80, console::Alignment::Center, None),
                console::pad_str_with("", 80, console::Alignment::Center, None, '-'),
            );
            println!("{}", console::style(string).red());
            std::process::exit(1);
        }
    }
}
