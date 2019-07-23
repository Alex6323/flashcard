//! A display for the terminal.
use crate::constants::{APP_NAME, APP_VERSION, HEADER_HEIGHT};
use crate::validator::{HintMode, LineValidator};

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
    width: usize,
    height: usize,
}

impl Display {
    /// Creates a new display.
    pub fn new() -> Self {
        let terminal = crossterm::terminal();
        let cursor = crossterm::cursor();
        let input = crossterm::input();
        let raw = RawScreen::into_raw_mode().expect("error switching to raw mode");

        let (width, height) = terminal.terminal_size();

        Self {
            terminal,
            cursor,
            input,
            raw,
            width: width as usize,
            height: height as usize,
        }
    }

    /// Clears the complete terminal. Should be called early.
    pub fn clear(&self) {
        self.terminal.clear(ClearType::All).expect("error clearing display");
        self.hide_cursor();
    }

    /// Clears everything except the header.
    pub fn clear_except_header(&self) {
        self.cursor.goto(0, HEADER_HEIGHT).expect("error moving cursor");
        self.terminal.clear(ClearType::FromCursorDown).expect("error clearing display");
    }

    /// Prints useful information about this cardbox.
    pub fn print_header(&self) {
        //
        let name_version = format!(" {} {}", APP_NAME, APP_VERSION);
        let name_version_x = self.width as u16 / 2 - name_version.len() as u16 / 2;

        print_frame_top(self.width);
        print_frame_mid(self.width);
        print_frame_mid(self.width);
        print_frame_mid(self.width);
        print_frame_bot(self.width);

        self.cprint_at(name_version, name_version_x, 2, Color::DarkBlue);
    }

    /// Reads input from user.
    pub fn read_input(&mut self, validator: &mut LineValidator) -> String {
        self.show_cursor();

        let mut reader = self.input.read_sync();

        'outer: loop {
            for c in reader.next() {
                match c {
                    InputEvent::Keyboard(e) => match e {
                        KeyEvent::Char(c) if c as u8 == 10 => (), //Ignore <ENTER>
                        KeyEvent::Char(c) => {
                            // if the user starts typing remove the hint if shown
                            match validator.hint_mode {
                                HintMode::Active(_) => {
                                    self.clear_hint(validator);
                                }
                                _ => (),
                            }

                            // only allow typing if the validator still accepts more
                            // characters
                            if validator.is_accepting() {
                                if validator.check(c) {
                                    self.cprint(c, Color::Green);
                                } else {
                                    self.cprint(c, Color::Red);
                                }
                            }
                        }
                        KeyEvent::Ctrl(c) if c == 'c' => {
                            self.exit();
                            process::exit(0);
                        }
                        KeyEvent::Backspace => {
                            match validator.hint_mode {
                                HintMode::Active(_) => {
                                    self.clear_hint(validator);
                                }
                                _ => (),
                            }

                            //if !chars.is_empty() {
                            if validator.index > 0 {
                                validator.undo(1);
                                self.cursor.move_left(1);
                                self.terminal
                                    .clear(ClearType::UntilNewLine)
                                    .expect("error clearing display");
                            }
                        }
                        KeyEvent::F(n) if n == 10 => {
                            match validator.hint_mode {
                                HintMode::Inactive => {
                                    //
                                    self.clear_incorrect(validator);

                                    // go back to the last correct char or to index 0
                                    self.cursor
                                        .save_position()
                                        .expect("error saving position");
                                }
                                _ => (),
                            }
                            if let Some(c) = validator.hint() {
                                self.cprint(c, Color::Yellow);
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
                if validator.is_happy() {
                    self.println_cr("");
                    break 'outer;
                }
            }
        }

        let line = validator.get_expected();

        self.hide_cursor();
        line
    }

    /// Prints text to the terminal without newline character.
    pub fn print(&self, text: impl std::fmt::Display) {
        self.terminal.write(format!("{}", text)).expect("error writing to terminal");
    }

    /// Prints text to the terminal without newline character after carriage return.
    pub fn print_cr(&self, text: impl std::fmt::Display) {
        self.terminal.write(format!("\r{}", text)).expect("error writing to terminal");
    }

    /// Prints colored text to the terminal without newline character.
    pub fn cprint(&self, text: impl std::fmt::Display, color: Color) {
        print!("{}{}{}", Colored::Fg(color), text, Colored::Fg(Color::Reset));
    }

    /// Prints colored text to the terminal without newline character after carriage
    /// return.
    pub fn cprint_cr(&self, text: impl std::fmt::Display, color: Color) {
        print!("\r{}{}{}", Colored::Fg(color), text, Colored::Fg(Color::Reset));
    }

    /// Prints text to the terminal with a newline character after carriage return.
    pub fn println_cr(&self, text: impl std::fmt::Display) {
        self.terminal.write(format!("\r{}\n", text)).expect("error writing to terminal");
    }

    /// Prints colored text to the terminal at a certain position.
    pub fn cprint_at(&self, text: impl std::fmt::Display, x: u16, y: u16, color: Color) {
        let (ox, oy) = self.cursor.pos();

        self.cursor.goto(x, y).expect("couldn't move cursor");
        print!("{}{}{}", Colored::Fg(color), text, Colored::Fg(Color::Reset));

        self.cursor.goto(ox, oy).expect("couldn't move cursor");
    }

    /// Ignores all input except <RETURN> and <CRTL-C>
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

    /// This function is used to remove the hint once the user starts typing again
    fn clear_hint(&self, validator: &mut LineValidator) {
        self.cursor.reset_position().expect("error resetting postion");

        self.terminal
            .clear(ClearType::UntilNewLine)
            .expect("error clearing rest of line");

        validator.hint_close();
    }

    /// This function is used to remove all correct/incorrect characters after the first
    /// incorrect character
    fn clear_incorrect(&mut self, validator: &mut LineValidator) {
        if let Some(first_incorrect) = validator.first_incorrect() {
            let delta = validator.index - first_incorrect;

            validator.undo(delta);

            if delta > 0 {
                self.cursor.move_left(delta as u16);
            }

            self.terminal
                .clear(ClearType::UntilNewLine)
                .expect("error clearing rest of line");
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.exit();
    }
}

fn cprintln_frame(text: &str, color: Color, width: usize) {
    print!("\r║{}", Colored::Fg(color));
    print!("{: <1$}", text, width - 1);
    println!("{}║", Colored::Fg(Color::Reset));
}

fn print_frame_top(width: usize) {
    println!("\r╔{:═<1$}╗", "", width - 1);
}

fn print_frame_mid(width: usize) {
    println!("\r║{: <1$}║", "", width - 1);
}

fn print_frame_bot(width: usize) {
    println!("\r╚{:═<1$}╝", "", width - 1);
}
