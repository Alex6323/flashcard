use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::cardbox::{FlashCard, Line, List};
use crate::constants::MARKUP_FACE;

struct FlashCardFactory {
    face: Option<Line>,
    back: List,
}

impl FlashCardFactory {
    pub fn new() -> Self {
        Self { face: None, back: vec![] }
    }

    pub fn add_face(&mut self, line: &str) {
        self.face = Some(String::from(line));
    }

    pub fn add_back(&mut self, line: &str) {
        //self.back = Some(String::from(line));
        self.back.push(String::from(line));
    }

    pub fn build(&mut self) -> FlashCard {
        let face = std::mem::replace(&mut self.face, None);

        let back = self.back.clone();
        self.back.clear();

        FlashCard { face: String::from(face.unwrap()), back }
    }

    pub fn reset(&mut self) {
        self.face = None;
        self.back.clear();
    }
}

/// A parser state machine.
#[derive(Clone, Copy, Eq, PartialEq)]
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
                ParserState::Back => true,
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

        let prev_state = state;

        //println!("{}", line);
        match line.chars().nth(0) {
            Some(first) if first == MARKUP_FACE => {
                if prev_state == ParserState::Back {
                    flashcards.push(factory.build());
                }
                state.move_to(ParserState::Face);
                let face_text = line.split(MARKUP_FACE).nth(1).unwrap().trim();
                factory.add_face(face_text);
            }
            Some(first) if first != MARKUP_FACE => {
                state.move_to(ParserState::Back);
                factory.add_back(line.trim());
            }
            _ => {
                if prev_state == ParserState::Back {
                    flashcards.push(factory.build());
                }
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
