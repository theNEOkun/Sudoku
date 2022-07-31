mod board;
mod position;

use std::{error::Error, io};

use board::Board;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout, Direction, Rect, Alignment},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, BorderType},
    Frame, Terminal,
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
                (val + 1).to_string()
            } else {
                if let Some(val) = self.app.board.tries[self.row][self.col] {
                    (val + 1).to_string()
                } else {
                    String::from("_")
                }
            }
        )
    }
}

struct App {
    board: Board,
    active_column: isize,
    active_row: isize,
}

impl App {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            active_column: 5,
            active_row: 5,
        }
    }

    fn up(&mut self) {
        self.active_row = if self.active_row - 1 > -1 {
            self.active_row - 1
        } else {
            0
        }
    }

    fn down(&mut self) {
        self.active_row = if self.active_row + 1 < (crate::board::SIDE as isize) {
            self.active_row + 1
        } else {
            crate::board::SIDE as isize - 1
        }
    }

    fn left(&mut self) {
        self.active_column = if self.active_column - 1 > -1 {
            self.active_column - 1
        } else {
            0
        }
    }

    fn right(&mut self) {
        self.active_column = if self.active_column + 1 < (crate::board::SIDE as isize) {
            self.active_column + 1
        } else {
            crate::board::SIDE as isize - 1
        }
    }

    fn active(&self) -> (usize, usize) {
        (self.active_row as usize, self.active_column as usize)
    }

    fn enter(&mut self, digit: usize) -> bool {
        let (row, col) = self.active();
        self.board.add_number(col, row, digit)
    }
}

fn board<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Rect {
        x: ((f.size().width) - 54) / 2,
        y: f.size().y + 2,
        width: 54,
        height: 27,
    };

    let large_cells = split_in_3x3(rects);

    for (r, row_rect) in large_cells.into_iter().enumerate() {
        let col_rects = split_in_3x3(row_rect);

        for (c, col_rect) in col_rects.into_iter().enumerate() {
            let (c, r) = square_to_point(r, c);
            let cell = Cell::new(app, r, c);
            let text = format!(" {} ", cell);

            let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(cell.block()).style(cell.text_style());

            f.render_widget(paragraph, col_rect);
        }
    };
}

/// Function to split a field into 3x3
///
/// see [MitchelPaulin](https://github.com/MitchelPaulin/sudoku-rs/blob/main/src/ui.rs) for
/// implementaiton
fn square_to_point(square_number: usize, cell_numbe: usize) -> (usize, usize) {
    let col = (square_number % 3) * 3 + cell_numbe % 3;
    let row = (square_number / 3) * 3 + cell_numbe / 3;

    (col, row)
}

/// Function to split a field into 3x3
///
/// see [MitchelPaulin](https://github.com/MitchelPaulin/sudoku-rs/blob/main/src/ui.rs) for
/// implementaiton
///
/// ## Arguments
///
/// * area - The area to split
///
/// ## Returns
///
/// a Vec of rects defining the split area
fn split_in_3x3(area: Rect) -> Vec<Rect> {
    let mut ret_rects = vec![];

    let rows = split_rect_in_3(area, Direction::Vertical);
    for row in rows {
        ret_rects.extend(split_rect_in_3(row, Direction::Horizontal));
    }
    ret_rects
}

fn split_rect_in_3(area: Rect, dir: Direction) -> Vec<Rect> {
    Layout::default()
        .direction(dir)
        .constraints(
            [
            Constraint::Ratio(1,3),
            Constraint::Ratio(1,3),
            Constraint::Ratio(1,3),
            ].as_ref(),
        )
        .split(area)
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
                KeyCode::Char('1') => {
                    app.enter(1);
                }
                KeyCode::Char('2') => {
                    app.enter(2);
                }
                KeyCode::Char('3') => {
                    app.enter(3);
                }
                KeyCode::Char('4') => {
                    app.enter(4);
                }
                KeyCode::Char('5') => {
                    app.enter(5);
                }
                KeyCode::Char('6') => {
                    app.enter(6);
                }
                KeyCode::Char('7') => {
                    app.enter(7);
                }
                KeyCode::Char('8') => {
                    app.enter(8);
                }
                KeyCode::Char('9') => {
                    app.enter(9);
                }
                KeyCode::Char('0') => {
                    app.enter(0);
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
