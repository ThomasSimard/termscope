use std::io::{self, BufRead, stderr, stdout, IsTerminal};
use std::time::{Duration, Instant};

use ratatui::{Frame};
use ratatui::{prelude::*};

use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};
use ratatui::crossterm::{
    execute,
    cursor,
    event::Event,
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub mod min_max;

mod data_processing;
mod data_parsing;

use data_parsing::parse_line;
use data_parsing::DataPoint;

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
    app()?;

    Ok(())
}

fn read_data(tx: &Sender<DataPoint>) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line: String = line.expect("failed to read line");

        if !io::stdout().is_terminal() {
            println!("{}", &line);
        }

        if let Ok(Some(data)) = parse_line(line){
            match tx.send(data) {
                Ok(_) => (),
                Err(_) => return,
            }
        }
    }
}

fn app() -> std::io::Result<()> {
    let (tx, rx): (Sender<DataPoint>, Receiver<DataPoint>) = mpsc::channel();

    thread::spawn(move || {
        loop {
            read_data(&tx);
        }
    });

    let mut stdout = stdout();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

    let mut processing = Processing::default();

    let mut last_draw = Instant::now();
    let draw_interval = Duration::from_millis(16);

    loop {
        while let Ok(data) = rx.try_recv() {
            processing.process(data);

            if last_draw.elapsed() >= draw_interval {
                break;
            }
        }

        terminal.draw(|frame| {
            render(frame, &processing)
        })?;


        if crossterm::event::poll(Duration::from_millis(0))? 
            && let Event::Key(_) = crossterm::event::read()? {
            break;
        }
        

        last_draw = Instant::now();
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show, cursor::EnableBlinking)?;

    Ok(())
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
