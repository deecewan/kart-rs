use dialoguer::{theme::ColorfulTheme, Select};
use image::DynamicImage;
use log::error;
use log_err::LogErrResult;

use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{ApiBackend, CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::{query, Camera};

pub fn from_device<F>(on_frame: F)
where
    F: 'static + Send + Sync + Fn(&DynamicImage, usize),
{
    let index = get_device();
    
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let mut camera = Camera::new(index, requested).log_expect("Could not create camera");
    
    camera.open_stream().log_expect("Could not open stream");
    
    let mut count = 0;
    loop {
        match camera.frame() {
            Ok(frame) => {
                count += 1;
                match frame.decode_image::<RgbFormat>() {
                    Ok(decoded) => {
                        let (width, height) = decoded.dimensions();
                        let raw = decoded.into_raw();
                        let buffer = image::ImageBuffer::from_raw(width, height, raw).unwrap();
                        let dynamic_image = DynamicImage::ImageRgb8(buffer);
                        on_frame(&dynamic_image, count);
                    }
                    Err(e) => {
                        error!("Error decoding frame: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error capturing frame: {:?}", e);
            }
        }
    }
}

fn get_device() -> CameraIndex {
    let cameras = query(ApiBackend::Auto).log_expect("Failed to query cameras");
    
    if cameras.is_empty() {
        let string = console::style("Couldn't find any capture devices.").red();
        error!("{string}");
        std::process::exit(1);
    }
    
    let options: Vec<String> = cameras
        .iter()
        .map(|info| {
            format!("{} ({})", info.human_name(), info.description())
        })
        .collect();
        
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a device:")
        .items(&options)
        .default(0)
        .interact();
        
    match selection {
        Ok(s) => {
            cameras[s].index().clone()
        }
        Err(e) => {
            error!("Error with device selection: {:?}", e);
            std::process::exit(1);
        }
    }
}
