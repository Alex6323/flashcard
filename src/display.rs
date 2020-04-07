//! A display for the terminal.
use crate::cardbox::Progress;
use crate::constants::BLANK_INDICATOR;
use crate::constants::NUM_REVEALED_CHARS_IN_HINT;
use crate::constants::{APP_NAME, APP_VERSION, HEADER_HEIGHT};
use crate::constants::{BG_COLOR, FG_COLOR};
use crate::constants::{PROGRAM_PEEK_KEY, PROGRAM_QUIT_KEY};
use crate::constants::{PROMPT_INPUT, PROMPT_WIDTH};
use crate::flashcards::*;
use crate::validator::{HintMode, InputValidator};

#[cfg(not(debug_assertions))]
use crossterm::AlternateScreen;
#[cfg(debug_assertions)]
use crossterm::RawScreen;

use crossterm::Colored;
use crossterm::{ClearType, Terminal, TerminalCursor, TerminalInput};
use crossterm::{InputEvent, KeyEvent};

// Re-export Color
pub use crossterm::Color;

/// Represents a cursor position the program expects an input.
type InputLocation = (u16, u16);

/// Represents all cursor positions the program expects an input.
struct InputLocations
{
    pub locations: Vec<InputLocation>,
    index: usize,
    length: usize,
}

impl InputLocations
{
    // Creates a new instance.
    //
    // Wherever `text` contains a `BLANK_INDICATOR` it will create an `InputLocation`.
    pub fn new(x: u16, y: u16, w: u16, h: u16, text: &str) -> Self
    {
        let mut locations = vec![];

        let num_chars = text.len() as u16;

        // Count required lines
        let mut num_lines = num_chars / w;
        if num_chars % w != 0 {
            num_lines += 1;
        }

        let mut u = x;
        let mut v = y;

        for (i, c) in text.chars().enumerate() {
            let i = i as u16;

            // Add input location
            if c == BLANK_INDICATOR {
                locations.push((u, v));
            }

            //
            if (i + 1) % w == 0 {
                // newline
                u = x;
                v += 1;
            } else {
                u += 1;
            }
        }

        let length = locations.len();

        Self { locations, index: 0, length }
    }

    /// Moves to the first cursor location.
    pub fn first(&mut self) -> InputLocation
    {
        assert!(!self.locations.is_empty());

        self.index = 0;
        self.locations[0]
    }

    /// Moves to the next cursor location.
    pub fn next(&mut self) -> Option<InputLocation>
    {
        if self.index < self.length {
            self.index += 1;
            if self.index < self.length {
                Some(self.locations[self.index])
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Moves to the previous cursor locationl
    pub fn prev(&mut self) -> Option<InputLocation>
    {
        if self.index > 0 {
            self.index -= 1;
            Some(self.locations[self.index])
        } else {
            None
        }
    }
}

/// Realizes a terminal based UI for this application.
pub struct Display
{
    terminal: Terminal,
    cursor: TerminalCursor,
    input: TerminalInput,
    #[cfg(not(debug_assertions))]
    _alt: AlternateScreen,
    #[cfg(debug_assertions)]
    _raw: crossterm::RawScreen,
    width: usize,
    height: usize,
}

impl Display
{
    /// Creates a new display.
    pub fn new() -> Self
    {
        #[cfg(not(debug_assertions))]
        let _alt = AlternateScreen::to_alternate(true)
            .expect("error creating alternate raw screen");
        #[cfg(debug_assertions)]
        let _raw = RawScreen::into_raw_mode().expect("error creating raw screen");

        let terminal = crossterm::terminal();
        let cursor = crossterm::cursor();
        let input = crossterm::input();

        let (width, height) = terminal.terminal_size();

        Self {
            terminal,
            cursor,
            input,
            #[cfg(not(debug_assertions))]
            _alt,
            #[cfg(debug_assertions)]
            _raw,
            width: width as usize,
            height: height as usize,
        }
    }

    /// Initializes the display.
    pub fn init(&self)
    {
        self.hide_cursor();
        self.clear();
        self.print_header();
        self.print_footer();
        self.cursor.goto(0, HEADER_HEIGHT).expect("error moving cursor");
    }

    /// Redraws the display.
    pub fn redraw(&self)
    {
        self.clear_input_area();
        self.cursor.goto(0, HEADER_HEIGHT).expect("error moving cursor");
    }

    /// Prints the header of this display.
    pub fn print_header(&self)
    {
        self.print_bar_top(BG_COLOR, self.width);
        self.print_title(FG_COLOR, BG_COLOR);

        // one empty line (just for style)
        println!();
    }

    /// Prints the footer of this display.
    pub fn print_footer(&self)
    {
        self.print_bar_bot(BG_COLOR, self.width);
        self.print_shortcuts(FG_COLOR, BG_COLOR);
    }

    /// Prints a notification that the flashcard was correctly answered.
    pub fn print_passed(&self)
    {
        let x = 1;
        let y = self.height as u16 - 1;
        self.cprint_at("Passed", x, y, Color::Green, Color::Reset);
    }

    /// Prints a notification that the flashcard was not correctly answered.
    pub fn print_failed(&self)
    {
        let x = 1;
        let y = self.height as u16 - 1;
        self.cprint_at("Failed", x, y, Color::Red, Color::Reset);
    }

    fn print_bar_top(&self, bg_color: Color, width: usize)
    {
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

    fn print_bar_bot(&self, bg_color: Color, width: usize)
    {
        self.cursor.save_position().expect("error saving cursor position");
        self.cursor.goto(0, self.height as u16).expect("error moving cursor");
        let empty_line = format!("{: <1$}", "", width + 1);
        print!("\r{}{}{}", Colored::Bg(bg_color), empty_line, Colored::Bg(Color::Reset));
        self.cursor.reset_position().expect("error resetting cusor position");
    }

    fn print_title(&self, fg: Color, bg: Color)
    {
        let name_version = format!("{} {}", APP_NAME, APP_VERSION);
        let x = 1;
        self.cprint_at(name_version, x, 0, fg, bg);
    }

    fn print_shortcuts(&self, fg: Color, bg: Color)
    {
        let shortcuts = format!(
            "{} | {} | {}",
            "RETURN: next flashcard", "CTRL-Q: quit program", "CTRL-P: peek at solution"
        );
        let x = 1;
        let y = self.height as u16;

        self.cprint_at(shortcuts, x, y, fg, bg);
    }

    /// Prints the progress.
    pub fn print_progress(&self, progress: Progress)
    {
        let stages = format!(
            "|{}|{}|{}|{}|{}| left: {}",
            progress.1, progress.2, progress.3, progress.4, progress.5, progress.0
        );
        let w = stages.len();
        let x = self.width as u16 - w as u16;

        self.cprint_at(stages, x, 0, FG_COLOR, BG_COLOR);
    }

    /// Reads input from the user and validates it against the given line validator.
    pub fn read_input_blanks(&mut self, validator: &mut InputValidator) -> bool
    {
        //let context = validator.context();
        let context = String::new();

        // Print prompt and context
        //self.print_cr(format!("{} {}", PROMPT_INPUT, context));

        let x = PROMPT_WIDTH + 1;

        let (_, y) = self.cursor.pos();
        let (w, h) = (self.width as u16, self.height as u16);

        self.cursor.goto(x, y).expect("error moving cursor");

        if cfg!(debug_assertions) {
            self.cprint_at(
                format!("x={}, y={}, w={}, h={}", x, y, w, h),
                1,
                self.height as u16 - 1,
                Color::Black,
                Color::White,
            );
        }

        // Based on terminal size, cursor position calculate a position for each character
        // that needs to be entered by the user
        let mut locations = InputLocations::new(x, y, w, h, &context);

        if cfg!(debug_assertions) {
            self.cprint_at(
                format!("locations={:?}", locations.locations),
                30,
                self.height as u16 - 1,
                Color::Black,
                Color::White,
            );
        }

        let (x, y) = locations.first();
        self.cursor.goto(x, y).expect("error moving cursor");

        self.show_cursor();
        let mut reader = self.input.read_sync();

        'outer: loop {
            for input in reader.next() {
                match input {
                    InputEvent::Keyboard(e) => match e {
                        // IGNORING
                        KeyEvent::Char(c) if c as u8 == 10 => (), // Ignore <ENTER>

                        // WRITING
                        KeyEvent::Char(c) => {
                            // only allow typing if the validator still accepts more
                            // characters
                            if validator.accepts() {
                                if validator.check(c) {
                                    self.cprint(c, Color::Green);
                                } else {
                                    self.cprint(c, Color::Red);
                                }
                            }

                            if let Some((x, y)) = locations.next() {
                                self.cursor.goto(x, y).expect("error moving cursor");
                            }
                        }

                        // QUITTING
                        KeyEvent::Ctrl(c) if c == PROGRAM_QUIT_KEY => {
                            self.exit();
                            return false;
                        }

                        // UNDOING
                        KeyEvent::Backspace => {
                            if validator.index() > 0 {
                                validator.undo(1);

                                if let Some((x, y)) = locations.prev() {
                                    /*
                                    self.cprint_at(
                                        "undo",
                                        1,
                                        self.height as u16 - 1,
                                        Color::White,
                                        Color::Blue,
                                    );
                                    */
                                    self.cursor.goto(x, y).expect("error moving cursor");
                                    self.print(BLANK_INDICATOR);
                                    self.cursor.goto(x, y).expect("error moving cursor");
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }

        self.hide_cursor();
        true
    }

    /// Reads input from the user and validates it against the given line validator.
    ///
    /// Returns `true` if valid input has been read. This function only returns `false`,
    /// if the program has been instructed to exit.
    pub fn read_input(&mut self, validator: &mut InputValidator) -> bool
    {
        self.show_cursor();

        let mut reader = self.input.read_sync();

        'outer: loop {
            for input in reader.next() {
                match input {
                    InputEvent::Keyboard(e) => match e {
                        // IGNORING
                        KeyEvent::Char(ch) if ch as u8 == 10 => (), //Ignore <ENTER>

                        // WRITING
                        KeyEvent::Char(ch) => {
                            // if the user starts typing remove the hint if shown
                            match validator.hint_mode() {
                                HintMode::Active(_) => {
                                    self.clear_hint(validator);
                                }
                                _ => (),
                            }

                            // only allow typing if the validator still accepts more
                            // characters
                            if validator.accepts() {
                                if validator.check(ch) {
                                    self.cprint(ch, Color::Green);
                                } else {
                                    self.cprint(ch, Color::Red);
                                }
                            }
                        }

                        // QUITTING
                        KeyEvent::Ctrl(ch) if ch == PROGRAM_QUIT_KEY => {
                            self.exit();
                            return false;
                        }

                        // UNDOING
                        KeyEvent::Backspace => {
                            match validator.hint_mode() {
                                HintMode::Active(_) => {
                                    self.clear_hint(validator);
                                }
                                _ => (),
                            }

                            if validator.index() > 0 {
                                validator.undo(1);
                                // BUG: if multiline, move cursor up and to the right
                                self.cursor.move_left(1);
                                self.terminal
                                    .clear(ClearType::UntilNewLine)
                                    .expect("error clearing display");
                            }
                        }

                        // PEEKING
                        KeyEvent::Ctrl(c) if c == PROGRAM_PEEK_KEY => {
                            match validator.hint_mode() {
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
                                if let Some(c) = validator.peek() {
                                    self.cprint(c, Color::Yellow);
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
                if validator.happy() {
                    self.println_cr("");
                    break 'outer;
                }
            }
        }

        self.hide_cursor();
        true
    }

    /// Prints text to the terminal without newline character.
    pub fn print(&self, text: impl std::fmt::Display)
    {
        self.terminal.write(format!("{}", text)).expect("error writing to terminal");
    }

    /// Prints text to the terminal without newline character after carriage return.
    pub fn print_cr(&self, text: impl std::fmt::Display)
    {
        self.terminal.write(format!("\r{}", text)).expect("error writing to terminal");
    }

    /// Prints colored text to the terminal without newline character.
    pub fn cprint(&self, text: impl std::fmt::Display, color: Color)
    {
        print!("{}{}{}", Colored::Fg(color), text, Colored::Fg(Color::Reset));
    }

    /// Prints colored text to the terminal without newline character after carriage
    /// return.
    pub fn cprint_cr(&self, text: impl std::fmt::Display, color: Color)
    {
        print!("\r{}{}{}", Colored::Fg(color), text, Colored::Fg(Color::Reset));
    }

    /// Prints colored text to the terminal without newline character after carriage
    /// return.
    pub fn cprintln_cr(&self, text: impl std::fmt::Display, color: Color)
    {
        println!("\r{}{}{}", Colored::Fg(color), text, Colored::Fg(Color::Reset));
    }

    /// Prints text to the terminal with a newline character after carriage return.
    pub fn println_cr(&self, text: impl std::fmt::Display)
    {
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
    )
    {
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
    pub fn wait_for_return(&self) -> bool
    {
        let mut reader = self.input.read_sync();
        'outer: loop {
            for c in reader.next() {
                match c {
                    InputEvent::Keyboard(e) => match e {
                        KeyEvent::Char(c) if c as u8 == 10 => break 'outer, // <RETURN>
                        KeyEvent::Ctrl(c) if c == PROGRAM_QUIT_KEY => {
                            self.exit();
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

    fn exit(&self)
    {
        RawScreen::disable_raw_mode().expect("error disabling raw-mode");
        self.show_cursor();
    }

    fn hide_cursor(&self)
    {
        self.cursor.hide().expect("error hiding cursor");
    }

    fn show_cursor(&self)
    {
        self.cursor.show().expect("error showing cursor");
    }

    /// Clears the complete terminal. Should be called early.
    fn clear(&self)
    {
        self.terminal.clear(ClearType::All).expect("error clearing display");
    }

    /// Clears everything except the header.
    fn clear_input_area(&self)
    {
        self.cursor.goto(2, HEADER_HEIGHT).expect("error moving cursor");
        self.clear_until_footer();
    }

    /// Clears until the footer begins.
    ///
    /// This method doesn't clear the footer so it doesn't need to be redrawn.
    fn clear_until_footer(&self)
    {
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
    fn clear_hint(&self, validator: &mut InputValidator)
    {
        self.cursor.reset_position().expect("error resetting postion");

        self.clear_until_footer();

        validator.end_peek();
    }

    /// This function is used to remove all correct/incorrect characters after the first
    /// incorrect character
    fn clear_incorrect(&mut self, validator: &mut InputValidator)
    {
        if let Some(first_incorrect) = validator.first_incorrect() {
            let delta = validator.index() - first_incorrect;

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

impl Drop for Display
{
    fn drop(&mut self)
    {
        self.exit();
    }
}
