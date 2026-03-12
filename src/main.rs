use ratatui::{DefaultTerminal, Frame};
use std::io::{self, BufRead};

mod data;
use data::parse_line;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(app)?;

    Ok(())
}


fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut dataset: Vec<(f64, f64)> = Vec::new();

    let stdin = io::stdin();

    loop {
        for line in stdin.lock().lines() {
            if let Ok(Some(data)) = parse_line(line?){
                dataset.push(data);
            }
        }

        terminal.draw(render)?;
        
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, dataset: Vec<(f64, f64)>) {
    frame.render_widget("hello world", frame.area());
}
