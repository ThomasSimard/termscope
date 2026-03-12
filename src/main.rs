use std::io::{self, BufRead};

use ratatui::{DefaultTerminal, Frame};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};

mod parser;
use parser::parse_line;

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

        terminal.draw(|frame| {
            render(frame, &dataset)
        })?;
        
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, dataset: &Vec<(f64, f64)>) {
    let chart_dataset = Dataset::default()
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Blue)
        .data(dataset);

    let x_axis = Axis::default()
        .title("x axis".blue())
        .bounds([0.0, 10.0])
        .labels(["0", "5", "10"]);

    let y_axis = Axis::default()
        .title("y axis".blue())
        .bounds([0.0, 10.0])
        .labels(["0", "5", "10"]);

    let chart = Chart::new(vec![chart_dataset]).x_axis(x_axis).y_axis(y_axis);

    frame.render_widget(chart, frame.area());
}
