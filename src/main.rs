use std::io::{self, BufRead, stderr, stdout, IsTerminal};

use ratatui::{Frame};
use ratatui::{prelude::*};

use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};
use ratatui::crossterm::{
    execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub mod min_max;

mod data_processing;
mod data_parsing;

use data_parsing::parse_line;

use crate::data_processing::Processing;

/*use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("100M_log.csv")?;
    let mut writer = BufWriter::new(file);

    for i in 1u32..100_000_000 {
        writeln!(writer, "{},{}", i, i.ilog10())?;
    }

    Ok(())
}*/

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    execute!(stderr(), Clear(ClearType::All))?;
    app();

    Ok(())
}

fn read_data(tx: Sender<(f64, f64)>) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line: String = line.expect("failed to read line");

        if !io::stdout().is_terminal() {
            println!("{}", &line);
        }

        if let Ok(Some(data)) = parse_line(line){
            //processing.process(data);
            tx.send(data).expect("failed to send data via channel");
        }
    }
}

fn app() {
    let (tx, rx): (Sender<(f64, f64)>, Receiver<(f64, f64)>) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let tx = tx.clone();

            read_data(tx);
        }
    });

    thread::spawn(move || -> std::io::Result<()> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

        let mut processing = Processing::default();
        
        loop {
            let data = rx.recv().expect("failed to receive data via channel");

            processing.process(data);

            terminal.draw(|frame| {
                render(frame, &processing)
            })?;

            if crossterm::event::read()?.is_key_press() {
                break;
            }
        }

        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    });
}

fn render(frame: &mut Frame, processing: &Processing) {
    let chart_dataset = Dataset::default()
        .name(format!("Dataset #1 ({})", processing.get_data().len()))
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Blue)
        .data(processing.get_data());

    let x_axis = Axis::default()
        .title("x axis".blue())
        .bounds([
            processing.domain.get_minimum(),
            processing.domain.get_maximum()
        ])
        .labels([
            processing.domain.get_minimum().to_string(),
            processing.domain.get_maximum().to_string(),
        ]);

    let y_axis = Axis::default()
        .title("y axis".blue())
        .bounds([
            processing.range.get_minimum(),
            processing.range.get_maximum() 
        ])
        .labels([
            processing.range.get_minimum().to_string(),
            processing.range.get_maximum().to_string(),
        ]);

    let chart = Chart::new(vec![chart_dataset]).x_axis(x_axis).y_axis(y_axis);

    frame.render_widget(chart, frame.area());
}
