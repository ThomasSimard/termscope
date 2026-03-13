use ratatui::{Frame};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};

use crate::data_processing::Processing;
use crate::min_max::MinMax;

fn generate_axis<'a>(min_max: &MinMax) -> Axis<'a> {
    Axis::default()
        .title("x axis".blue())
        .bounds([
            min_max.get_minimum(),
            min_max.get_maximum()
        ])
        .labels([
            min_max.get_minimum().to_string(),
            min_max.get_maximum().to_string(),
        ])
}

fn generate_chart_dataset<'a>(data: &'a [(f64, f64)]) -> Dataset<'a> {
    Dataset::default()
        .name(format!("Dataset ({})", data.len()))
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Blue)
        .data(data)
}

pub fn render(frame: &mut Frame, processing: &Processing) {
    let x_axis = generate_axis(&processing.domain);
    let y_axis = generate_axis(&processing.range); 

    let mut datasets = Vec::default();

    for data in processing.get_data() {
        datasets.push(generate_chart_dataset(data));
    }

    let chart = Chart::new(datasets).x_axis(x_axis).y_axis(y_axis);

    frame.render_widget(chart, frame.area());
}
