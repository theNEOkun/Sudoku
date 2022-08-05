use std::io::{stdout, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal, Frame};

pub struct Term {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    /// On startup creates and sets everything needed to control the terminal as needed
    pub fn new() -> Self {
        let backend = CrosstermBackend::new(stdout());
        let term = Self {
            terminal: Terminal::new(backend).unwrap(),
        };
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        term
    }

    /// used to render to the terminal
    ///
    /// Takes a function, that writes to the frame
    ///
    /// ## Arguments
    ///
    /// * fun - A function used to render to terminal
    pub fn render(&mut self, fun: &mut dyn FnMut(&mut Frame<CrosstermBackend<Stdout>>)) {
        self.terminal.draw(|frame| {
            fun(frame);
        }).unwrap();
    }
}

impl Drop for Term {
    /// Drops the terminal, so everything used to start the terminal is removed
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        self.terminal.show_cursor().unwrap();
    }
}
