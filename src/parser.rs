use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::cardbox::{FlashCard, Line, List};
use crate::constants::{MARKUP_FACE, MARKUP_NOTE, MARKUP_META};

/// A factory to produce flashcards from markup.
struct FlashCardFactory {
    face: Option<Line>,
    back: List,
    note: Option<Line>,
}

impl FlashCardFactory {
    /// Creates a new factory.
    pub fn new() -> Self {
        Self { face: None, back: vec![], note: None }
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
            face: String::from(face.unwrap()),
            back,
            note,
        }
    }

    /// Resets the factory so it can be reused to produce another flashcard.
    pub fn reset(&mut self) {
        self.face = None;
        self.back.clear();
        self.note = None;
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

pub fn parse(path: &str) -> Vec<FlashCard> {
    let path = Path::new(path);
    let file = File::open(&path).unwrap();
    let buff = BufReader::new(file);

    let mut flashcards = vec![];
    let mut state = ParserState::Init;
    let mut factory = FlashCardFactory::new();

    for line in buff.lines().filter_map(|r| r.ok()) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        //println!("{}", line);

        // 1st char must exist, so unwrap won't fail ever
        match line.chars().nth(0).unwrap() {
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
                state.move_to(ParserState::Back);
                factory.add_back(line.trim());
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

    #[test]
    fn parse_sample_lession() {
        let path = Path::new("./sample_box.txt");
    }
}
