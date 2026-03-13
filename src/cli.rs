use clap::{Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Column of the X axis
    #[arg(long, default_value_t = 1)]
    pub x: usize,

    /// Column of the Y axis
    #[arg(long, default_value_t = 2)]
    pub y: usize,
}
