//! A display for the terminal.

use crossterm::RawScreen;
use crossterm::{ClearType, Terminal, TerminalCursor, TerminalInput};
use crossterm::{Color, Colored};
use crossterm::{InputEvent, KeyEvent};

use std::process;

/// Represents a display to display flashcards.
pub struct Display {
    terminal: Terminal,
    cursor: TerminalCursor,
    input: TerminalInput,
    raw: RawScreen,
    width: u16,
    height: u16,
}

impl Display {
    /// Creates a new display.
    pub fn new() -> Self {
        let terminal = crossterm::terminal();
        let cursor = crossterm::cursor();
        let input = crossterm::input();
        let raw = RawScreen::into_raw_mode().expect("error switching to raw mode");

        let (width, height) = terminal.terminal_size();

        Self { terminal, cursor, input, raw, width, height }
    }

    /// Clears the complete terminal. Should be called early.
    pub fn clear(&self) {
        self.terminal.clear(ClearType::All).expect("error clearing display");
        self.hide_cursor();
    }

    /// Prints useful information about this cardbox.
    pub fn print_header(&self) {
        //
    }

    /// Reads input from user.
    pub fn read_input(&mut self) -> String {
        self.show_cursor();

        let mut reader = self.input.read_sync();

        let mut chars = vec![];

        'outer: loop {
            for c in reader.next() {
                match c {
                    InputEvent::Keyboard(e) => match e {
                        KeyEvent::Char(c) if c as u8 == 10 => {
                            self.println("");
                            break 'outer
                        }
                        KeyEvent::Char(c) => {
                            chars.push(c);
                            self.print(c);
                        }
                        KeyEvent::Ctrl(c) if c == 'c' => {
                            self.exit();
                            process::exit(0);
                        }
                        KeyEvent::Backspace => {
                            chars.pop();
                            self.cursor.move_left(1);
                            self.terminal.clear(ClearType::UntilNewLine).expect("error clearing display");
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }

        let line = chars.iter().collect::<String>();

        self.hide_cursor();
        line
    }

    /// Prints text to the terminal without newline character.
    pub fn print(&self, text: impl std::fmt::Display) {
        self.terminal.write(format!("{}", text)).expect("error writing to terminal");
    }

    /// Prints text to the terminal with a newline character.
    pub fn println(&self, text: impl std::fmt::Display) {
        self.terminal.write(format!("\r{}\n", text)).expect("error writing to terminal");
    }

    ///
    pub fn wait_for_return(&self) {
        let mut reader = self.input.read_sync();
        'outer: loop {
            for c in reader.next() {
                match c {
                    InputEvent::Keyboard(e) => match e {
                        KeyEvent::Char(c) if c as u8 == 10 => break 'outer,
                        KeyEvent::Ctrl(c) if c == 'c' => {
                            self.exit();
                            process::exit(0);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

    fn exit(&self) {
        RawScreen::disable_raw_mode().expect("error disabling raw-mode");
        self.show_cursor();
    }

    fn hide_cursor(&self) {
        self.cursor.hide().expect("error hiding cursor");
    }

    fn show_cursor(&self) {
        self.cursor.show().expect("error showing cursor");
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.exit();
    }
}
