use ratatui::{Frame};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};

use crate::data_processing::Processing;

pub fn render(frame: &mut Frame, processing: &Processing) {
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
