use std::io::{stderr, stdout};
use std::time::{Duration, Instant};

use ratatui::{prelude::*};
use ratatui::crossterm::{
    execute,
    cursor,
    event::Event,
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread};

use crate::data_parsing::Parser;
use crate::data_processing::Processing;

use crate::input::read_data;
use crate::ui::render;

use crate::DataPoint;

pub fn app() -> std::io::Result<()> {
    execute!(stderr(), Clear(ClearType::All))?;

    let (tx, rx): (Sender<DataPoint>, Receiver<DataPoint>) = mpsc::channel();

    let parser = Parser::default();

    thread::spawn(move || {
        loop {
            read_data(&parser, &tx);
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

