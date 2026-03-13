pub mod min_max;

mod cli;
mod app;
mod ui;
mod input;

mod data_processing;
mod data_parsing;

use app::app;

pub type DataPoint = (f64, f64);

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    app()?;

    Ok(())
}

