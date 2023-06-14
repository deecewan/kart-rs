use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Error,
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub vendor_id: Option<String>,

    #[arg(short, long)]
    pub product_id: Option<String>,

    #[arg(long)]
    pub store_frames: bool,

    #[arg(long, value_enum, default_value_t = LogLevel::Error)]
    pub log: LogLevel,

    #[arg(long)]
    pub store_logs: bool,
}
