use std::io::{Stderr, stderr, stdout};

use std::time::Duration;

use ratatui::crossterm::{
    cursor,
    event::Event,
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::prelude::*;

use color_eyre::eyre::Result;

use crate::data_processing::Processing;
use crate::ui::ui::UI;

use crate::ui::message;
use crate::ui::render;

pub struct Ratatui {
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl Default for Ratatui {
    fn default() -> Self {
        let terminal_result = Terminal::new(CrosstermBackend::new(stderr()));

        let terminal = terminal_result.expect("failed to initialize terminal");

        Self { terminal }
    }
}

impl UI for Ratatui {
    fn init(&mut self) -> Result<()> {
        execute!(stderr(), Clear(ClearType::All))?;

        enable_raw_mode()?;

        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        Ok(())
    }

    fn main_screen(&mut self, processing: &Processing) -> Result<()> {
        self.terminal.draw(|frame| render(frame, &processing))?;

        Ok(())
    }

    fn waiting_screen(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            message::render(frame, "Waiting for data...");
        })?;

        Ok(())
    }

    fn handle_input(&mut self) -> bool {
        if crossterm::event::poll(Duration::from_millis(0)).expect("failed to wait")
            && let Event::Key(_) = crossterm::event::read().expect("failed to read key")
        {
            return true;
        }

        false
    }

    fn cleanup(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            cursor::Show,
            cursor::EnableBlinking
        )?;

        Ok(())
    }
}
