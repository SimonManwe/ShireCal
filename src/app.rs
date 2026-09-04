use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::components::grid::Grid;

#[derive(Debug, Default)]
pub struct App {
    quit_application: bool,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            quit_application: false,
        })
    }

    pub fn run_application(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.quit_application {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let grid = Grid::new();

        frame.render_widget(grid, frame.area());
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit_application(),
            _ => {}
        }
    }

    fn quit_application(&mut self) {
        self.quit_application = true;
    }
}
