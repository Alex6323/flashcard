//! A display for the terminal.
use crate::automat::Progress;
use crate::constants::NUM_REVEALED_CHARS_IN_HINT;
use crate::constants::{APP_NAME, APP_VERSION, HEADER_HEIGHT};
use crate::constants::{BG_COLOR, FG_COLOR};
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
    _raw: RawScreen,
    width: usize,
    height: usize,
}

impl Display {
    /// Creates a new display.
    pub fn new() -> Self {
        let terminal = crossterm::terminal();
        let cursor = crossterm::cursor();
        let input = crossterm::input();
        let _raw = RawScreen::into_raw_mode().expect("error switching to raw mode");

        let (width, height) = terminal.terminal_size();

        Self {
            terminal,
            cursor,
            input,
            _raw,
            width: width as usize,
            height: height as usize,
        }
    }

    /// Initializes the display.
    pub fn init(&self) {
        self.hide_cursor();
        self.clear();
        self.print_header();
        self.print_footer();
        self.cursor.goto(0, HEADER_HEIGHT).expect("error moving cursor");
    }

    /// Redraws the display.
    pub fn redraw(&self) {
        self.clear_input_area();
        self.cursor.goto(0, HEADER_HEIGHT).expect("error moving cursor");
    }

    /// Prints the header of this display.
    pub fn print_header(&self) {
        self.print_bar_top(BG_COLOR, self.width);
        self.print_title();
        println!(); // one empty line
    }

    /// Prints the footer of this display.
    pub fn print_footer(&self) {
        self.print_bar_bot(BG_COLOR, self.width);
        self.print_shortcuts();
    }

    fn print_bar_top(&self, bg_color: Color, width: usize) {
        self.cursor.save_position().expect("error saving cursor position");
        self.cursor.goto(0, 0).expect("error moving cursor");
        let empty_line = format!("{: <1$}", "", width + 1);
        println!(
            "\r{}{}{}",
            Colored::Bg(bg_color),
            empty_line,
            Colored::Bg(Color::Reset)
        );
        self.cursor.reset_position().expect("error resetting cusor position");
    }

    fn print_bar_bot(&self, bg_color: Color, width: usize) {
        self.cursor.save_position().expect("error saving cursor position");
        self.cursor.goto(0, self.height as u16).expect("error moving cursor");
        let empty_line = format!("{: <1$}", "", width + 1);
        print!("\r{}{}{}", Colored::Bg(bg_color), empty_line, Colored::Bg(Color::Reset));
        self.cursor.reset_position().expect("error resetting cusor position");
    }

    fn print_title(&self) {
        let name_version = format!("{} {}", APP_NAME, APP_VERSION);
        let x = 1;
        self.cprint_at(name_version, x, 0, FG_COLOR, BG_COLOR);
    }

    fn print_shortcuts(&self) {
        let shortcuts = format!("{} | {}", "CTRL-C: exit program", "CTRL-H: show hint");
        let x = 1;
        let y = self.height as u16;

        self.cprint_at(shortcuts, x, y, FG_COLOR, BG_COLOR);
    }

    /// Prints the progress.
    pub fn print_progress(&self, progress: Progress) {
        let stages = format!(
            "{}|{}|{}|{}|{}|{}",
            progress.0, progress.1, progress.2, progress.3, progress.4, progress.5
        );
        let w = stages.len();
        let x = self.width as u16 - w as u16;

        self.cprint_at(stages, x, 0, FG_COLOR, BG_COLOR);
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

                            if validator.index > 0 {
                                validator.undo(1);
                                self.cursor.move_left(1);
                                self.terminal
                                    .clear(ClearType::UntilNewLine)
                                    .expect("error clearing display");
                            }
                        }
                        KeyEvent::Ctrl(c) if c == 'h' => {
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
                            for _ in 0..NUM_REVEALED_CHARS_IN_HINT {
                                if let Some(c) = validator.hint() {
                                    self.cprint(c, Color::Yellow);
                                }
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
    pub fn cprint_at(
        &self,
        text: impl std::fmt::Display,
        x: u16,
        y: u16,
        fg_color: Color,
        bg_color: Color,
    ) {
        let (ox, oy) = self.cursor.pos();

        self.cursor.goto(x, y).expect("couldn't move cursor");
        print!(
            "{}{}{}{}{}",
            Colored::Bg(bg_color),
            Colored::Fg(fg_color),
            text,
            Colored::Fg(Color::Reset),
            Colored::Bg(Color::Reset)
        );

        self.cursor.goto(ox, oy).expect("couldn't move cursor");
    }

    /// Ignores all input except <RETURN> and <CRTL-C>
    pub fn wait_for_return(&self) -> bool {
        let mut reader = self.input.read_sync();
        'outer: loop {
            for c in reader.next() {
                match c {
                    InputEvent::Keyboard(e) => match e {
                        KeyEvent::Char(c) if c as u8 == 10 => break 'outer, // <RETURN>
                        KeyEvent::Ctrl(c) if c == 'c' => {
                            self.exit();
                            //process::exit(0);
                            return true;
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        false
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

    /// Clears the complete terminal. Should be called early.
    fn clear(&self) {
        self.terminal.clear(ClearType::All).expect("error clearing display");
    }

    /// Clears everything except the header.
    fn clear_input_area(&self) {
        self.cursor.goto(2, HEADER_HEIGHT).expect("error moving cursor");
        self.clear_until_footer();
    }

    /// Clears until the footer begins.
    ///
    /// This method doesn't clear the footer so it doesn't need to be redrawn.
    fn clear_until_footer(&self) {
        let (_, current_y) = self.cursor.pos();
        self.cursor.save_position().expect("error saving cursor position");
        self.terminal.clear(ClearType::UntilNewLine).expect("error clearing line");
        for y in (current_y + 1)..(self.height as u16) {
            self.cursor.goto(0, y).expect("error moving cursor");
            self.terminal.clear(ClearType::CurrentLine).expect("error clearing line");
        }
        self.cursor.reset_position().expect("error restoring cursor position");
    }

    /// This function is used to remove the hint once the user starts typing again
    fn clear_hint(&self, validator: &mut LineValidator) {
        self.cursor.reset_position().expect("error resetting postion");

        self.clear_until_footer();

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
