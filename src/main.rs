mod board;
mod position;

use std::{error::Error, io};

use board::Board;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

struct App {
    state: TableState,
    board: Board,
    active_column: usize,
    active_row: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            board: Board::new(),
            active_column: 0,
            active_row: 0,
        }
    }

    fn up(&mut self) {
        if let Some(active_row) = self.active_row.checked_sub(1) {
            self.active_row = active_row;
        }
    }

    fn down(&mut self) {
        self.active_row += usize::from(self.active_row < 9);
    } 

    fn left(&mut self) {
        if let Some(active_column) = self.active_column.checked_sub(1) {
            self.active_column = active_column;
        }
    }

    fn right(&mut self) {
        self.active_column += usize::from(self.active_column < 9);
    }

    pub fn select(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.board.empty.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Min(50)].as_ref())
        .margin(5)
        .split(f.size());

    let rows = app.board.empty.iter().map(|item| {
        let cells = item.iter().map(|c| {
            Cell::from(if let Some(value) = c {
                value.to_string()
            } else {
                String::from(" ")
            })
            .style(Style::default().bg(Color::White).fg(Color::Black).add_modifier(Modifier::UNDERLINED))
        });
        Row::new(cells).height(3)
    });

    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);

    f.render_stateful_widget(t, rects[0], &mut app.state);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(())
                },
                KeyCode::Left => {
                    app.left();
                },
                KeyCode::Right => {
                    app.right();
                },
                KeyCode::Up => {
                    app.up();
                },
                KeyCode::Down => {
                    app.down();
                },
                KeyCode::Char(c) => {
                }
                _ => {}
            }
        }
    }
}

#[deny(clippy::pedantic)]
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let app = App::new();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // create app and run it
    let res = run_app(&mut terminal, app);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    terminal.show_cursor().unwrap();

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
