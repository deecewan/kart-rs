use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub vendor_id: Option<String>,

    #[arg(short, long)]
    pub product_id: Option<String>,

    #[arg(long)]
    pub store_frames: bool,
}
