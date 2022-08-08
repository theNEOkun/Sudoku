pub mod states;

use crossterm::event::{self, Event, KeyCode};
use std::io::{self, Stdout};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    board::{self, difficulties::Difficulties, Board},
    term::Term,
};

/// The size of each tile
const TILE_SIZE: u16 = 3;

/// The size of the entire sudoku-board
const SUDOKU_SIZE: u16 = TILE_SIZE * board::SIDE as u16;

/// Function to get the string value from the specific part of the board
///
/// ## Arguments
///
/// * row - The row to get from
/// * col - The column to get from
/// * board - The board to get the element from
///
/// ## Returns
///
/// (the value, boolean)
/// * The Value - is the value to get
/// * Boolean - The boolean is true if the value is in the "Empty" board, and not just in the
/// "tries" board
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

pub struct App {
    /// The board to act upon
    board: Board,
    /// The current active column position
    active_column: isize,
    /// The current active row position
    active_row: isize,
    /// The file-name to save to
    file_name: String,
}

impl App {
    pub fn new(difficulty: Difficulties, file: Option<String>) -> Self {
        if let Some(file) = file {
            let board = Board::from_string(
                std::fs::read_to_string(&file).expect("That file does not exist here"),
            );
            Self {
                board,
                active_column: (board::SIDE / 2) as isize,
                active_row: (board::SIDE / 2) as isize,
                file_name: file,
            }
        } else {
            Self {
                board: Board::new(&difficulty),
                active_column: (board::SIDE / 2) as isize,
                active_row: (board::SIDE / 2) as isize,
                file_name: format!("save-{}", difficulty),
            }
        }
    }

    /// Moves the active position up
    fn up(&mut self) {
        self.active_row = if (self.active_row - 1) > -1 {
            self.active_row - 1
        } else {
            (crate::board::SIDE - 1) as isize
        }
    }

    /// Moves the active position down
    fn down(&mut self) {
        self.active_row = if (self.active_row + 1) < (crate::board::SIDE as isize) {
            self.active_row + 1
        } else {
            0
        }
    }

    /// Moves the active position left
    fn left(&mut self) {
        self.active_column = if (self.active_column - 1) > -1 {
            self.active_column - 1
        } else {
            (crate::board::SIDE - 1) as isize
        }
    }

    /// Moves the active position right
    fn right(&mut self) {
        self.active_column = if (self.active_column + 1) < (crate::board::SIDE as isize) {
            self.active_column + 1
        } else {
            0
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
        self.board.add_number(col, row, digit)
    }
}

/// Used to get the block of the current cell
///
/// Changes based on if the cell is active or not
///
/// ## Arguments
///
/// * bg_color - The background-color to use
fn block<'a>(bg_color: Color) -> Block<'a> {
    let color = bg_color;
    Block::default()
        .style(Style::default().bg(color).fg(color))
        .borders(Borders::ALL)
        .border_style(Style::default())
        .border_type(BorderType::Plain)
}

/// Used to get the text-style of the current cell
///
/// The foreground color is based on if the number is in the "emtpy"-set or the "tries"-set
/// The background color is based on if the current cell is active or not, together with also
/// changing if the number should be bold or not
///
/// ## Arguments
///
/// * bg_color - The back-ground color to use
fn text_style(old: bool, is_active: bool, bg_color: Color) -> Style {
    Style::default()
        .fg(if old { Color::Blue } else { Color::Black })
        .bg(if is_active { Color::Cyan } else { bg_color })
        .add_modifier(if is_active {
            Modifier::BOLD
        } else {
            Modifier::empty()
        })
}

/// Sets up the board
///
/// ## Arguments
///
/// * f - is the frame to be written to
/// * app - is the app to be run from
fn board<B: Backend>(f: &mut Frame<B>, window: Rect, app: &mut App) {
    let rects = Rect {
        x: window.x + (SUDOKU_SIZE / 4),
        y: window.y + (SUDOKU_SIZE / 8),
        width: SUDOKU_SIZE * 2,
        height: SUDOKU_SIZE,
    };

    // Splits the alloted space into a 3x3
    let large_cells = split_in_3x3(rects);

    for (r, row_rect) in large_cells.into_iter().enumerate() {
        // Splits each rectangle into a 3x3
        let col_rects = split_in_3x3(row_rect);

        for (c, col_rect) in col_rects.into_iter().enumerate() {
            let bg_color = match r % 2 {
                0 => Color::Gray,
                _ => Color::White,
            };

            // Convert to "proper" axis
            let (c, r) = square_to_point(r, c);

            let (value, old) = get_string_value(r, c, &app.board);
            let text = format!(" {} ", value);

            let is_active = app.active() == (r, c);

            let paragraph = Paragraph::new(text)
                .alignment(Alignment::Center)
                .style(text_style(old, is_active, bg_color));

            let text_rect = Rect {
                x: col_rect.x + 1,
                y: col_rect.y + 1,
                width: 3,
                height: 1,
            };

            f.render_widget(block(bg_color), col_rect);
            f.render_widget(paragraph, text_rect);
        }

        //f.render_widget(
        //    Block::default()
        //        .borders(Borders::ALL)
        //        .border_style(Style::default().fg(Color::Black)),
        //    row_rect,
        //);
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

/// Used to write the information - window
///
/// ## Arguments
///
/// * f - The frame used to write into
/// * window - is the alloted window to be contained in
/// * status - is a bitflag of different statuses
fn info_window<B: Backend>(f: &mut Frame<B>, window: Rect, status: u8) {
    let rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(window);
    let info_par = Paragraph::new(vec![
        Spans::from(format!(
            "Is board correct?: {}\n",
            if status & 0x1 == 0x1 { "true" } else { "false" },
        )),
        Spans::from(format!(
            "Saved: {} \n",
            if status & 0b0110 == 0b0010 {
                "true"
            } else {
                "false"
            }
        )),
        Spans::from(format!(
            "Loaded: {} \n",
            if status & 0b0110 == 0b0100 {
                "true"
            } else {
                "false"
            }
        )),
        Spans::from(format!(
            "All positions correct: {} \n",
            if status & 0x40 == 0x40 {
                "true"
            } else {
                "false"
            }
        )),
        Spans::from(String::from("")),
    ])
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Left);
    let paragraph = Paragraph::new(vec![
        Spans::from(String::from("↑↓←→ for up/down/left/right")),
        Spans::from(String::from("1-9 for adding a number")),
        Spans::from(String::from("Space or 0 for removing a number")),
        Spans::from(String::from("S to save L to load")),
        Spans::from(String::from("Q to Close")),
    ])
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center);
    f.render_widget(info_par, rect[0]);
    f.render_widget(paragraph, rect[1]);
}

const CLEAR_FLAG: u8 = 0x1;

/// Handles the input of the keys
/// Placed here due to being quite the few
///
/// ## Arguments
/// * key - the key to match agains
/// * app - the app the handle onto
/// * status - the current status of the app
///
/// ## Returns
///
/// The status again, potentially changed
fn read_key(key: KeyCode, app: &mut App, status: u8) -> u8 {
    let mut status = status;
    match key {
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
            std::fs::write(&app.file_name, app.board.to_string()).expect("Write failed");
            status &= CLEAR_FLAG;
            status |= 0x2;
        }
        KeyCode::Char('l') => {
            app.board =
                Board::from_string(std::fs::read_to_string(&app.file_name).expect("No such file"));
            status &= CLEAR_FLAG;
            status |= 0x4;
        }
        KeyCode::Char('1') => {
            app.enter(1);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('2') => {
            app.enter(2);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('3') => {
            app.enter(3);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('4') => {
            app.enter(4);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('5') => {
            app.enter(5);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('6') => {
            app.enter(6);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('7') => {
            app.enter(7);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('8') => {
            app.enter(8);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('9') => {
            app.enter(9);
            status &= CLEAR_FLAG;
        }
        KeyCode::Char('0') | KeyCode::Char(' ') => {
            app.enter(0);
            status &= CLEAR_FLAG;
        }
        _ => {}
    }
    status
}

pub fn run_app(terminal: &mut Term, mut app: App) -> io::Result<()> {
    // Bitflags
    // 0x1 = Is the solution correct?
    // 0x2 = Is it saved?
    // 0x4 = Is it loaded
    // 0x20 = all positions filled
    // 0x40 = all positions correct
    let mut status: u8 = 0x0;
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
            info_window(f, center[1], status);
        });

        if let Event::Key(key) = event::read()? {
            status = match key.code {
                KeyCode::Char('q') => return Ok(()),
                rest => read_key(rest, &mut app, status),
            }
        }

        if app.board.test_board() {
            status |= 0x40;
        }

        if app.board.test_board() {
            status |= 0x1;
            break;
        }
    }
    loop {
        terminal.render(&mut |frame| {
            let message = format!("Congratulations, you won!");
            let rect = Rect {
                x: frame.size().x,
                y: frame.size().y,
                width: frame.size().width,
                height: frame.size().height,
            };
            let paragraph = Paragraph::new(vec![
                Spans::from(String::from("Q to Close")),
                Spans::from(message),
            ]);
            frame.render_widget(paragraph, rect);
        });

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}
