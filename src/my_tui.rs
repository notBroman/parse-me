use std::{fs, io, thread, time::Duration};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::*,
    widgets::{self, Block, Borders, List, ListItem, ListState},
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

struct MyItem {
    // `items` is the state managed by your application.
    items: Vec<String>,
    // `state` is the state that can be modified by the UI. It stores the index of the selected
    // item as well as the offset computed during the previous draw call (used to implement
    // natural scrolling).
    state: ListState,
}

impl MyItem {
    fn new(items: Vec<String>) -> MyItem {
        MyItem {
            items,
            state: ListState::default(),
        }
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        // We reset the state as the associated items have changed. This effectively reset
        // the selection as well as the stored offset.
        self.state = ListState::default();
    }

    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Unselect the currently selected item if any. The implementation of `ListState` makes
    // sure that the stored offset is also reset.
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub fn func() {
    println!("Hello from tui!");

    let _ = enable_raw_mode();
    let mut stdout = io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Error when creating terminal");

    let mut names: Vec<String> = vec![];

    for f in fs::read_dir("./data/").expect("Empty directory") {
        names.push(
            f.expect("Not an entry")
                .file_name()
                .to_str()
                .unwrap()
                .clone()
                .to_owned(),
        );
    }

    // use ListState to keep track of what is selected
    let mut items = MyItem::new(names);

    loop {
        let _ = terminal
            .draw(|f| ui(f, &mut items))
            .expect("Error when drawing box");

        match event::read() {
            Err(e) => {}
            Ok(event) => match event {
                Event::Key(key) => {
                    if key == KeyCode::Esc.into() {
                        break;
                    } else if key == KeyCode::Up.into() {
                        items.previous();
                    } else if key == KeyCode::Down.into() {
                        items.next();
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

pub fn ui<B: Backend>(f: &mut Frame<B>, items: &mut MyItem) {
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let block = Block::default().title("Header").borders(Borders::ALL);
    f.render_widget(block, chunk[0]);

    let itemsList: Vec<ListItem> = items
        .items
        .iter()
        .map(|i| ListItem::new(i.as_ref()))
        .collect();
    let list = List::new(itemsList)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">");
    f.render_stateful_widget(list, chunk[1], &mut items.state);
}
