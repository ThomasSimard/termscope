use color_eyre::eyre::Result;

use crate::data_processing::Processing;

pub trait UI {
    fn init(&mut self) -> Result<()>;

    fn main_screen(&mut self, processing: &Processing) -> Result<()>;

    fn waiting_screen(&mut self) -> Result<()>;

    fn handle_input(&mut self) -> bool;

    fn cleanup(&mut self) -> Result<()>;
}
