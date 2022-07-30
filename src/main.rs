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
    items: Board,
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Min(50)].as_ref())
        .margin(5)
        .split(f.size());

    let rows = app.items.empty.iter().map(|item| {
        let cells = item.iter().map(|c| {
            Cell::from(if let Some(value) = c.get_value() {
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut board: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut board));

        if let Event::Mouse(m) = event::read()? {
            match m.kind {
                MouseEventKind::Down(button) => {
                    match button => {
                        MouseButton::Left => {
                            println!("@ {}, {}", m.column, m.row);
                        }
                        _ => {}
                    }
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

    let app = App {
        state: TableState::default(),
        items: Board::new(),
    };

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
