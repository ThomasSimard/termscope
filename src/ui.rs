use ratatui::{Frame};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};

use crate::data_processing::Processing;
use crate::min_max::MinMax;

fn generate_axis<'a>(min_max: &MinMax, title: String) -> Axis<'a> {
    Axis::default()
        .title(title.blue())
        .bounds([
            min_max.get_minimum(),
            min_max.get_maximum()
        ])
        .labels([
            min_max.get_minimum().to_string(),
            min_max.get_maximum().to_string(),
        ])
}

const COLORS: [Color; 6] = [
    Color::Blue,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Magenta,
    Color::Cyan,
];

fn generate_chart_dataset<'a>(data: &'a [(f64, f64)], index: usize) -> Dataset<'a> {
    Dataset::default()
        .name(format!("dataset #{} ({})", index+1, data.len()))
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(COLORS[index])
        .data(data)
}

pub fn render(frame: &mut Frame, processing: &Processing) {
    let x_axis = generate_axis(&processing.domain, String::from("x axis"));
    let y_axis = generate_axis(&processing.range,String::from("y axis")); 

    let datasets: Vec<_> = processing
        .get_data()
        .iter()
        .enumerate()
        .map(|(i, data)| generate_chart_dataset(data, i))
        .collect();

    let chart = Chart::new(datasets).x_axis(x_axis).y_axis(y_axis);

    frame.render_widget(chart, frame.area());
}
