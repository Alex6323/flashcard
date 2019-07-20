use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::cardbox::FlashCard;
use crate::constants::MARKUP_FACE;

struct FlashCardFactory {
    face: Option<String>,
    back: Option<String>,
}

impl FlashCardFactory {
    pub fn new() -> Self {
        Self { face: None, back: None }
    }

    pub fn add_face(&mut self, face_text: &str) {
        self.face = Some(String::from(face_text));
    }

    pub fn add_back(&mut self, back_text: &str) {
        self.back = Some(String::from(back_text));
    }

    pub fn build(&mut self) -> FlashCard {
        let face = std::mem::replace(&mut self.face, None);
        let back = std::mem::replace(&mut self.back, None);

        FlashCard { face: String::from(face.unwrap()), back: String::from(back.unwrap()) }
    }

    pub fn reset(&mut self) {
        self.face = None;
        self.back = None;
    }
}

/// A parser state machine.
enum ParserState {
    Init,
    Face,
    Back,
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
            },
            ParserState::Face => match *next_state {
                ParserState::Init => true,
                ParserState::Face => false,
                ParserState::Back => true,
            },
            ParserState::Back => match *next_state {
                ParserState::Init => true,
                ParserState::Face => true,
                ParserState::Back => false,
            },
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
        // skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        match line.chars().nth(0) {
            Some(first) if first == MARKUP_FACE => {
                state.move_to(ParserState::Face);
                let face_text = line.split(MARKUP_FACE).nth(1).unwrap().trim();
                factory.add_face(face_text);
            }
            Some(first) if first != MARKUP_FACE => {
                state.move_to(ParserState::Back);
                let back_text = line.trim();
                factory.add_back(back_text);

                let flashcard = factory.build();
                flashcards.push(flashcard);
            }
            _ => {
                state.move_to(ParserState::Init);
                factory.reset();
            }
        }
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
