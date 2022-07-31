use std::io::{stdout, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal, Frame};

pub struct Term {
    stdout: Stdout,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn new() -> Self {
        let backend = CrosstermBackend::new(stdout());
        let term = Self {
            stdout: stdout(),
            terminal: Terminal::new(backend).unwrap(),
        };
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        term
    }

    pub fn render(&mut self, fun: &mut dyn FnMut(&mut Frame<CrosstermBackend<Stdout>>)) {
        self.terminal.draw(|frame| {
            fun(frame);
        }).unwrap();
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        self.terminal.show_cursor().unwrap();
    }
}
