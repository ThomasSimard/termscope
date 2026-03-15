use clap::{Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about="Plot data directly in the terminal",
    long_about = None,
    after_help = "Examples:
    termscope data.csv
    cat data.csv | termscope
    cat data.csv | termscope > save.csv
    "
)]
pub struct Cli {
    #[arg()]
    pub file: Option<PathBuf>,
    
    /// Column index used for the X axis
    #[arg(long, default_value_t = 1)]
    pub x: usize,

    /// Column indexes use for the Y series
    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    pub y: Vec<usize>,

    /// Delimiter used to seperate columns
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}
