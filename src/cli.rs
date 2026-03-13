use clap::{Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Column of the X axis
    #[arg(long, default_value_t = 1)]
    pub x: usize,

    /// Columns of the Y axis
    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    pub y: Vec<usize>,
}
