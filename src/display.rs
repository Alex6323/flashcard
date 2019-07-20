//! A display for the terminal.

use crossterm::{ClearType, Terminal, TerminalCursor};
use crossterm::{Color, Colored};

/// Represents a display to display flashcards.
pub struct Display {
    terminal: Terminal,
    cursor: TerminalCursor,
    width: u16,
    height: u16,
}

impl Display {
    /// Creates a new display.
    pub fn new() -> Self {
        let terminal = crossterm::terminal();
        let cursor = crossterm::cursor();
        let (width, height) = terminal.terminal_size();

        Self {
            terminal,
            cursor,
            width,
            height,
        }
    }

    /// Clears the complete terminal. Should be called early.
    pub fn clear(&self) {
        self.terminal.clear(ClearType::All).expect("error clearing display");
    }
}