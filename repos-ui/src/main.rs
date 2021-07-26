use anyhow::Result;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use log::LevelFilter;
use repos::repos;
use simplelog::{Config, WriteLogger};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Terminal;

struct FuzzyList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub filter: Option<String>,
}

impl<T> FuzzyList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
            filter: None,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i < self.items.len() - 1 => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let start_of_list = 0;
        let end_of_list = self.items.len() - 1;
        let i = match self.state.selected() {
            Some(i) if i == 0 => end_of_list,
            Some(i) => i - 1,
            _ => start_of_list,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App {
    items: FuzzyList<PathBuf>,
}

impl App {
    fn from_search(results: Vec<PathBuf>) -> Self {
        Self {
            items: FuzzyList::with_items(results),
        }
    }
}

fn main() -> Result<()> {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("repos-ui.log")?,
    )?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let settings = repos::Settings::from_env()?;
    let results = repos(&settings).collect::<Vec<_>>();
    let mut app = App::from_search(results);

    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let block = Block::default().title("Search").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default().title("Repositories").borders(Borders::ALL);
            let items: Vec<ListItem> = app
                .items
                .items
                .iter()
                .map(|x| ListItem::new(x.to_string_lossy().to_string()).style(Style::default()))
                .collect();
            let items = List::new(items)
                .block(block)
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");
            f.render_stateful_widget(items, chunks[1], &mut app.items.state);
        })?;

        match read()? {
            Event::Key(input) => match input {
                KeyEvent {
                    code: KeyCode::Down | KeyCode::Right,
                    modifiers: _,
                } => app.items.next(),
                KeyEvent {
                    code: KeyCode::Up | KeyCode::Left,
                    modifiers: _,
                } => app.items.previous(),
                _ => {
                    log::warn!("Unhandled: {:?}", input);
                }
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
}
