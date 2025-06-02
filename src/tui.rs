use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{self, Block, Borders},
    Frame, Terminal,
};

use crossterm::{
    event::{
        self, DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture,
        Event, KeyCode,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn func() {
    println!("Hello from tui!");

    let _ = enable_raw_mode();
    let mut stdout = io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Error when creating terminal");

    loop {
        let _ = terminal.draw(|f| ui(f)).expect("Error when drawing box");

        match event::read() {
            Err(e) => {}
            Ok(event) => match event {
                Event::Key(key) => {
                    if key == KeyCode::Esc.into() {
                        break;
                    }
                }
                _ => {}
            },
        };
    }

    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );

    let _ = terminal.show_cursor();
}

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunk[0]);

    let block2 = Block::default().title("Block2").borders(Borders::ALL);
    f.render_widget(block2, chunk[1]);
}
