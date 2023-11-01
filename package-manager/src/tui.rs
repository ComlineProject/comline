// Standard Uses
use std::{io, thread};
use std::time::Duration;

// Crate Uses

// External Uses
use eyre::Result;
use crossterm::{
    event, event::DisableMouseCapture, event::EnableMouseCapture,
    execute
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode,
    EnterAlternateScreen, LeaveAlternateScreen
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::{Block, Borders};


#[allow(unused)]
pub(crate) fn new_package() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    // Start a thread to discard any input events. Without handling events, the
    // stdin buffer will fill up, and be read into the shell when the program exits.
    thread::spawn(||
        loop { event::read().expect("Loop panicked"); }
    );
    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

