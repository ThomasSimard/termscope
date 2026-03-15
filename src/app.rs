use std::time::{Duration, Instant};

use clap::Parser;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use color_eyre::eyre::Result;

use crate::data_parsing::DataParser;
use crate::data_processing::Processing;

use crate::input::read_data;

use crate::ui::ratatui::Ratatui;

use crate::cli::Cli;
use crate::ui::ui::UI;

pub fn app() -> Result<()> {
    let cli = Cli::parse();

    let mut processing = Processing::default();

    let (tx, rx): (Sender<Vec<f64>>, Receiver<Vec<f64>>) = mpsc::channel();

    let parser = DataParser::new(cli.delimiter);

    thread::spawn(move || {
        read_data(&parser, &tx, &cli);
    });

    let mut last_draw = Instant::now();
    let draw_interval = Duration::from_millis(16);

    let mut ui = Ratatui::default();

    ui.init()?;

    ui.waiting_screen()?;

    loop {
        while let Ok(data) = rx.try_recv() {
            processing.init(data.len() - 1);
            processing.process(&data);

            if last_draw.elapsed() >= draw_interval {
                break;
            }
        }

        if !processing.get_data().is_empty() {
            ui.main_screen(&processing)?;
        }

        if ui.handle_input() {
            break;
        }

        last_draw = Instant::now();
    }

    ui.cleanup()?;
    Ok(())
}
