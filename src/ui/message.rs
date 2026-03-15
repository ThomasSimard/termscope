use ratatui::Frame;

pub fn render(frame: &mut Frame, message: &str) {
    frame.render_widget(message, frame.area());
}
