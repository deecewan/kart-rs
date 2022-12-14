use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};

mod cli;

fn main() {
    let args = cli::Cli::parse();

    let ctx = uvc::Context::new().expect("Could not get context");
    let device = match (args.vendor_id, args.product_id) {
        (Some(vendor_id), Some(product_id)) => {
            let vendor_id: i32 = vendor_id.parse().expect("failed to parse vendor_id");
            let product_id: i32 = product_id.parse().expect("failed to parse product_id");
            ctx.find_device(Some(vendor_id), Some(product_id), None)
                .expect("Could not find device")
        }
        _ => {
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
    };

    kart_rs::stream::from_device(&device);
}
