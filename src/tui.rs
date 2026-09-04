use std::io::{Stdout, stdout};

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture, EventStream, KeyEvent, MouseEvent},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};
use ratatui::backend::CrosstermBackend;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Key(KeyEvent),
    Mouse(MouseEvent),
}

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
    pub frame_rate: f64,
    pub tick_rate: f64,
}

impl Tui {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
            frame_rate: 60.0,
            tick_rate: 4.0,
        })
    }

    pub fn start_tui(&mut self) -> color_eyre::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            stdout(),
            SetTitle("ShireCal"),
            EnterAlternateScreen,
            EnableMouseCapture,
            cursor::Hide
        )?;

        Ok(())
    }

    pub fn teardown_tui(&mut self) -> color_eyre::Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        )?;
        ratatui::restore();

        Ok(())
    }

    async fn start_event_loop(
        event_tx: UnboundedSender<Event>,
        cancellation_token: CancellationToken,
        tick_rate: f64,
        frame_rate: f64,
    ) {
        let mut event_stream = EventStream::new();
    }
}
