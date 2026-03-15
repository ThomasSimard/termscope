pub mod min_max;

mod app;
mod cli;
mod input;

mod data_parsing;
mod data_processing;

use app::app;

pub mod ui;

pub type DataPoint = (f64, f64);

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    app()?;

    Ok(())
}
