mod app;
mod components;
mod tui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut app = app::App::new()?;

    let mut tui = tui::Tui::new()?;
    tui.start_tui()?;

    app.run_application(&mut tui.terminal)?;

    tui.teardown_tui()?;
    Ok(())
}
