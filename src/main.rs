mod board;
mod position;

use std::{error::Error, io};

use board::Board;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEvent,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState, Paragraph, BorderType},
    Frame, Terminal, text::{Span, Spans},
};

struct Cell<'a> {
    app: &'a App,
    row: usize,
    col: usize,
}

impl<'a> Cell<'a> {
    fn new(app: &'a App, row: usize, col: usize) -> Self {
        Self { app, row, col }
    }

    fn is_active(&self) -> bool {
        self.app.active() == (self.row, self.col)
    }

    fn position(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn block(&self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .bg(Color::Black)
                    .fg(if self.is_active() {
                        Color::Cyan
                    } else {
                        Color::White
                    })
                    .add_modifier(if self.is_active() {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            )
            .border_type(BorderType::Rounded)
    }

    fn text_style(&self) -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(if self.is_active() {
                Color::Cyan
            } else {
                Color::White
            })
    }
}

impl std::fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if let Some(val) = self.app.board[self.position()] {
                val.to_string()
            } else {
                String::from(" ")
            }
        )
    }
}

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
        self.active_row += usize::from(self.active_row < 8);
    }

    fn left(&mut self) {
        if let Some(active_column) = self.active_column.checked_sub(1) {
            self.active_column = active_column;
        }
    }

    fn right(&mut self) {
        self.active_column += usize::from(self.active_column < 8);
    }

    fn active(&self) -> (usize, usize) {
        (self.active_row, self.active_column)
    }
}

fn board<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Min(50)].as_ref())
        .margin(5)
        .split(f.size());

    let row_rects = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .horizontal_margin(0)
        .constraints(std::iter::repeat(Constraint::Length(3)).collect::<Vec<_>>())
        .split(rects[0]);

    for (r, row_rect) in row_rects.into_iter().enumerate() {

        let col_rects = Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(1)
            .horizontal_margin(0)
            .constraints(std::iter::repeat(Constraint::Length(3)).collect::<Vec<_>>())
            .split(row_rect);

        for (c, col_rect) in col_rects.into_iter().enumerate() {
            let cell = Cell::new(app, r, c);
            let text = format!("{} ", cell);

            let paragraph = Paragraph::new(text).block(cell.block()).style(cell.text_style());

            f.render_widget(paragraph, col_rect);
        }
    };
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let outer_block = Block::default().borders(Borders::ALL);
            f.render_widget(outer_block, f.size());

            board(f, &mut app)
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Left => {
                    app.left();
                }
                KeyCode::Right => {
                    app.right();
                }
                KeyCode::Up => {
                    app.up();
                }
                KeyCode::Down => {
                    app.down();
                }
                KeyCode::Char(c) => {}
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
