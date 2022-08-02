mod board;
mod position;
mod term;

use std::{
    error::Error,
    io::{self, Stdout},
};

use board::Board;

use crossterm::event::{self, Event, KeyCode};
use term::Term;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

struct Cell<'a> {
    /// The app itself, containting the board
    app: &'a App,
    /// The row of the specific cell (y-axis)
    row: usize,
    /// The column of the cell (x-axis)
    col: usize,
    /// If the value was prefilled or not
    old: bool,
    /// Which value was there
    value: String,
}

fn get_string_value(row: usize, col: usize, board: &Board) -> (String, bool) {
    // Is there a number in the "empty"-board?
    if let Some(val) = board.empty[row][col] {
        ((val + 1).to_string(), true)
    } else {
        // Is there a number in the board with tries?
        if let Some(val) = board[(row, col)] {
            ((val + 1).to_string(), false)
        } else {
            (String::from("_"), false)
        }
    }
}

impl<'a> Cell<'a> {
    fn new(app: &'a App, row: usize, col: usize) -> Self {
        let (value, old) = get_string_value(row, col, &app.board);
        Self {
            app,
            row,
            col,
            old,
            value,
        }
    }

    fn is_active(&self) -> bool {
        self.app.active() == (self.row, self.col)
    }

    fn block(&self) -> Block {
        Block::default().style(
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

    fn text_style(&self, bg_color: Color) -> Style {
        Style::default()
            .fg(if self.old { Color::Red } else { Color::Black })
            .bg(if self.is_active() {
                Color::Cyan
            } else {
                bg_color
            })
    }
}

impl std::fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct App {
    board: Board,
    active_column: isize,
    active_row: isize,
    filled_squares: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            active_column: (board::SIDE / 2) as isize,
            active_row: (board::SIDE / 2) as isize,
            filled_squares: 0,
        }
    }

    /// Moves the active position up
    fn up(&mut self) {
        self.active_row = if self.active_row - 1 > -1 {
            self.active_row - 1
        } else {
            0
        }
    }

    /// Moves the active position down
    fn down(&mut self) {
        self.active_row = if self.active_row + 1 < (crate::board::SIDE as isize) {
            self.active_row + 1
        } else {
            crate::board::SIDE as isize - 1
        }
    }

    /// Moves the active position left
    fn left(&mut self) {
        self.active_column = if self.active_column - 1 > -1 {
            self.active_column - 1
        } else {
            0
        }
    }

    /// Moves the active position right
    fn right(&mut self) {
        self.active_column = if self.active_column + 1 < (crate::board::SIDE as isize) {
            self.active_column + 1
        } else {
            crate::board::SIDE as isize - 1
        }
    }

    /// Getst the active position (y, x)
    fn active(&self) -> (usize, usize) {
        (self.active_row as usize, self.active_column as usize)
    }

    /// Enters a number at the active position
    ///
    /// ## Arguments
    ///
    /// * digit - the number to add
    ///
    /// ## Returns
    ///
    /// a boolean if it succeeded
    fn enter(&mut self, digit: usize) -> bool {
        let (row, col) = self.active();
        if self.board.add_number(col, row, digit) {
            self.filled_squares += 1;
            true
        } else {
            false
        }
    }

    /// Checks if the number of filled squares are the same as empty squares on the board
    ///
    /// ## Returns
    ///
    /// true if they are the same, else false"
    fn finished(&self) -> bool {
        if self.filled_squares == self.board.empty_squares {
            true
        } else {
            false
        }
    }
}

/// Sets up the board
///
/// ## Arguments
///
/// * f - is the frame to be written to
/// * app - is the app to be run from
fn board<B: Backend>(f: &mut Frame<B>, window: Rect, app: &mut App) {
    let rects = Rect {
        x: window.x,
        y: window.y,
        width: 54,
        height: 27,
    };

    let large_cells = split_in_3x3(rects);

    for (r, row_rect) in large_cells.into_iter().enumerate() {
        let col_rects = split_in_3x3(row_rect);

        for (c, col_rect) in col_rects.into_iter().enumerate() {
            let bg_color = match r % 2 {
                0 => Color::Gray,
                _ => Color::White,
            };

            let (c, r) = square_to_point(r, c);
            let cell = Cell::new(app, r, c);
            let text = format!(" {} ", cell);

            let paragraph = Paragraph::new(text)
                .alignment(Alignment::Center)
                .style(cell.text_style(bg_color));

            let text_rect = Rect {
                x: col_rect.x + 1,
                y: col_rect.y + 1,
                width: 4,
                height: 2,
            };

            f.render_widget(cell.block(), col_rect);
            f.render_widget(paragraph, text_rect);
        }
    }
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

/// Function to split a rectangle in 3
///
/// see [MitchelPaulin](https://github.com/MitchelPaulin/sudoku-rs/blob/main/src/ui.rs) for
/// implementaiton
///
/// ## Arguments
/// * area - is the Rectangle to split
/// * dir - is the direction to split in
///
/// ## Returns
/// a Vec of Rect
fn split_rect_in_3(area: Rect, dir: Direction) -> Vec<Rect> {
    Layout::default()
        .direction(dir)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(area)
}

fn info_window<B: Backend>(f: &mut Frame<B>, window: Rect, correct: bool) {
    let paragraph = Paragraph::new(Span::from(format!(
        "Is the board correct?: {}",
        if correct { "true" } else { "false" }
    )));
    f.render_widget(paragraph, window);
}

fn run_app(terminal: &mut Term, mut app: App) -> io::Result<()> {
    let mut correct = false;
    loop {
        terminal.render(&mut |f: &mut Frame<CrosstermBackend<Stdout>>| {
            let outer_block = Block::default().borders(Borders::ALL);
            f.render_widget(outer_block, f.size());
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(f.size());

            let center = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
                .split(layout[1]);

            board(f, center[0], &mut app);
            info_window(f, center[1], correct);
        });

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
                KeyCode::Char('s') => {
                    std::fs::write("saved", app.board.to_string())?;
                }
                KeyCode::Char('l') => {
                    app.board = Board::from_string(std::fs::read_to_string("saved")?);
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
                KeyCode::Char('0') | KeyCode::Char(' ') => {
                    app.enter(0);
                }
                _ => {}
            }
        }

        if app.finished() {
            correct = app.board.test_board();
        }

        if correct {
            return Ok(());
        }
    }
}

#[deny(clippy::pedantic)]
fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    let mut terminal = Term::new();

    // create app and run it
    let res = run_app(&mut terminal, app);

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
