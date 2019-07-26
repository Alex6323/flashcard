//! Functionality for parsing flashcard text files.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

use crate::constants::{MARKUP, MARKUP_ESCAPE, MARKUP_FACE, MARKUP_META, MARKUP_NOTE};
use crate::flashcard::{FlashCard, Line, List};

/// A factory to produce flashcards from markup.
struct FlashCardFactory {
    subject: Rc<String>,
    face: Option<Line>,
    back: List,
    note: Option<Line>,
}

impl FlashCardFactory {
    /// Creates a new factory.
    pub fn new(subject: &str) -> Self {
        Self {
            subject: Rc::new(String::from(subject)),
            face: None,
            back: vec![],
            note: None,
        }
    }

    /// Adds the front side of the flashcard.
    pub fn add_face(&mut self, line: &str) {
        self.face = Some(String::from(line));
    }

    /// Adds the back side of the flashcard.
    pub fn add_back(&mut self, line: &str) {
        self.back.push(String::from(line));
    }

    /// Adds a note to the flashcard to provide helpful context.
    pub fn add_note(&mut self, line: &str) {
        self.note = Some(String::from(line));
    }

    /// Returns `true` if enough data has been provided to build a flashcard.
    pub fn can_build(&self) -> bool {
        self.face.is_some() && !self.back.is_empty()
    }

    /// Builds a flashcard from the current state of the factory.
    pub fn build(&mut self) -> FlashCard {
        assert!(self.can_build());

        let face = std::mem::replace(&mut self.face, None);

        let back = self.back.clone();
        self.back.clear();

        let note = std::mem::replace(&mut self.note, None);

        FlashCard {
            subject: Rc::clone(&self.subject),
            face: String::from(face.unwrap()),
            back,
            note,
        }
    }
}

/// A parser state machine.
#[derive(Clone, Copy, Eq, PartialEq)]
enum ParserState {
    Init,
    Face,
    Back,
    Note,
}

impl ParserState {
    pub fn move_to(&mut self, state: ParserState) {
        if !self.can_move_to(&state) {
            panic!("cannot parse file");
        }
        *self = state;
    }

    fn can_move_to(&self, next_state: &ParserState) -> bool {
        match *self {
            ParserState::Init => match *next_state {
                ParserState::Init => true,
                ParserState::Face => true,
                ParserState::Back => false,
                ParserState::Note => false,
            },
            ParserState::Face => match *next_state {
                ParserState::Init => true,
                ParserState::Face => false,
                ParserState::Back => true,
                ParserState::Note => false,
            },
            ParserState::Back => match *next_state {
                ParserState::Init => true,
                ParserState::Face => true,
                ParserState::Back => true,
                ParserState::Note => true,
            },
            ParserState::Note => match *next_state {
                ParserState::Init => true,
                ParserState::Face => true,
                ParserState::Back => false,
                ParserState::Note => false,
            },
        }
    }

    fn can_build(&self, next_state: &ParserState) -> bool {
        match *self {
            ParserState::Back => match *next_state {
                ParserState::Init => true,
                ParserState::Face => true,
                ParserState::Back => false,
                ParserState::Note => false,
            },
            ParserState::Note => match *next_state {
                ParserState::Init => true,
                ParserState::Face => true,
                ParserState::Back => false,
                ParserState::Note => false,
            },
            _ => false,
        }
    }
}

/// Parses the flashcard text file to a `FlashCard` instances.
pub fn parse(path: &str) -> Vec<FlashCard> {
    let path = Path::new(path);
    let file = File::open(&path).unwrap();
    let buff = BufReader::new(file);
    let name = path.file_name().unwrap().to_str().unwrap();

    let mut flashcards = vec![];
    let mut state = ParserState::Init;
    let mut factory = FlashCardFactory::new(name);

    for line in buff.lines().filter_map(|r| r.ok()) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        //println!("{}", line);

        // 1st char must exist, so unwrap won't fail ever
        let first_char = line.chars().nth(0).unwrap();

        match first_char {
            MARKUP_FACE => {
                if state.can_build(&ParserState::Face) {
                    flashcards.push(factory.build());
                }

                state.move_to(ParserState::Face);
                let face_text = line.split(MARKUP_FACE).nth(1).unwrap().trim();
                factory.add_face(face_text);
            }
            MARKUP_NOTE => {
                state.move_to(ParserState::Note);
                let note_text = line.split(MARKUP_NOTE).nth(1).unwrap().trim();
                factory.add_note(note_text);
            }
            MARKUP_META => (), // Ignore this line
            _ => {
                // If the 1st character is the `Escape` character, and actually used for
                // escaping a markup char ...
                if first_char == MARKUP_ESCAPE
                    && if let Some(c) = line.chars().nth(1) {
                        MARKUP.contains(&c)
                    } else {
                        false
                    }
                {
                    // ... then remove it, and treat the rest of the line as part of the
                    // flashcard data
                    factory.add_back(&line.chars().skip(1).collect::<String>());
                } else {
                    factory.add_back(line);
                }

                state.move_to(ParserState::Back);
            }
        }
    }

    // Is there one last flashcard in the factory that can be built?
    if factory.can_build() {
        flashcards.push(factory.build());
    }

    flashcards
}

#[cfg(test)]
mod tests {
    use super::*;

}
