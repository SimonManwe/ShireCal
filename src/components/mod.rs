use ratatui::{
    Frame,
    layout::{Rect, Size},
};

pub mod grid;

pub trait Component {
    fn init(&mut self, area: Size) -> color_eyre::Result<()> {
        let _ = area; // stfu clippy
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()>;
}
