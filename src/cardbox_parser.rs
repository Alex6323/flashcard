//! Functionality for parsing flashcard text files.

use crate::constants::{MARKUP, MARKUP_COMMENT, MARKUP_ESCAPE, MARKUP_FACE, MARKUP_NOTE};
use crate::flashcards::flashcard_factory::FlashcardFactory;
use crate::flashcards::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A parser state machine.
#[derive(Clone, Copy, Eq, PartialEq)]
enum ParserState
{
    Init,
    Face,
    Back,
    Note,
}

impl ParserState
{
    pub fn move_to(&mut self, state: ParserState)
    {
        if !self.can_move_to(&state) {
            panic!("cannot parse file");
        }
        *self = state;
    }

    fn can_move_to(&self, next_state: &ParserState) -> bool
    {
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

    fn can_build(&self, next_state: &ParserState) -> bool
    {
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

/// Parses a cardbox from the given file.
pub fn parse_from_file(path: &str) -> Vec<Flashcard>
{
    let path = Path::new(path);
    let file = File::open(&path).unwrap();
    let buff = BufReader::new(file);
    let name = path.file_name().unwrap().to_str().unwrap();
    parse(buff, name)
}

/// Parses a cardbox from the given reader and a subject name.
pub fn parse(buff: impl BufRead, name: &str) -> Vec<Flashcard>
{
    use crate::constants::*;
    use crate::flashcards::FlashcardBack::*;

    let mut flashcards = vec![];
    let mut state = ParserState::Init;
    let mut factory = FlashcardFactory::new(name);
    let mut card_back = None;

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
                    let back = std::mem::replace(&mut card_back, None).unwrap();
                    factory.set_back(back);
                    flashcards.push(factory.build());
                }

                state.move_to(ParserState::Face);

                if line.len() < 2 {
                    panic!("error: no flashcard front specified");
                }

                // Determine the type of flashcard according to the number '#'
                let count = line.chars().take_while(|c| c == &MARKUP_FACE).count();
                let face_text = line.chars().skip(count).collect::<String>();
                //println!("count = {}, face_text = {}", count, face_text);

                factory.set_face(face_text.trim());

                card_back = match count {
                    TYPE_WRITE_THE_LINE => Some(WriteTheLine(vec![])),
                    TYPE_FILL_THE_BLANK => Some(FillTheBlank(vec![])),
                    _ => panic!("flashcard type is not supported"),
                };
            }

            MARKUP_NOTE => {
                state.move_to(ParserState::Note);

                let note_text = line.split(MARKUP_NOTE).nth(1).unwrap().trim();

                factory.set_note(note_text);
            }

            MARKUP_COMMENT => (), // Ignore this line

            _ => {
                let data = {
                    // If the 1st character is the `Escape` character, and actually used
                    // for escaping a markup char ...
                    if first_char == MARKUP_ESCAPE
                        && if let Some(c) = line.chars().nth(1) {
                            MARKUP.contains(&c)
                        } else {
                            false
                        }
                    {
                        // ... then remove it, and treat the rest of the line as part of
                        // the flashcard data
                        line.chars().skip(1).collect::<String>()
                    } else {
                        line.to_string()
                    }
                };

                // Depending on the card type parse the back of the flashcard
                match card_back.as_mut() {
                    Some(v) => match v {
                        WriteTheLine(lines) => {
                            //println!("Pushing data = {}", data);
                            lines.push(data);
                        }
                        FillTheBlank(lines) => {
                            // UPPERCASE indicates a blank
                            // '<','>' alternative way to indicate a blank
                            // '_' indicates that the following letter has to be entered
                            //      as uppercase as well
                            // '\' indicates that the following uppercase word is not
                            // actually a blank
                            let mut parts: LineWithBlanks = vec![];
                            let mut index = 0;

                            for part in data.split_whitespace() {
                                //
                                let blank = part
                                    .chars()
                                    .all(|c| c.is_uppercase() || !c.is_alphanumeric());

                                let p = if blank {
                                    String::from(part.to_lowercase())
                                } else {
                                    String::from(part)
                                };

                                parts.push(LinePart(p, blank, index));

                                index += part.len() + 1;
                            }

                            lines.push(parts);
                        }
                    },
                    None => panic!("error parsing the file"),
                }
                state.move_to(ParserState::Back);
            }
        }
    }

    let back = std::mem::replace(&mut card_back, None).unwrap();
    factory.set_back(back);
    // Is there one last flashcard in the factory that can be built?
    //println!("can be built = {}", factory.can_build());
    if factory.can_build() {
        flashcards.push(factory.build());
    }

    flashcards
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_one_write_the_line_flashcard()
    {
        #[rustfmt::skip]
        let s = Cursor::new("
        % This is a comment
        # WriteTheLine
            this is a write-the-line flashcard
            ! And this is a note.

        ");

        assert_eq!(1, parse(s, "test").len());
    }

    #[test]
    fn parse_two_write_the_line_flashcards()
    {
        #[rustfmt::skip]
        let s = Cursor::new("
        # WriteTheLine 1
            this is a write-the-line flashcard

        # WriteTheLine 2
            this is another write-the-line flashcard
        ");

        assert_eq!(2, parse(s, "test").len());
    }

    #[test]
    fn parse_one_fill_the_blank_flashcard()
    {
        #[rustfmt::skip]
        let s = Cursor::new("
        ## FillTheBlank
            this is a FILL_THE_BLANK flashcard
        ");

        assert_eq!(1, parse(s, "test").len());
    }

    #[test]
    fn parse_two_fill_the_blank_flashcards()
    {
        #[rustfmt::skip]
        let s = Cursor::new("
        ## FillTheBlank 1
            this is a FILL_THE_BLANK flashcard

        ## FillTheBlank 2
            this is another FILL_THE_BLANK flashcard
        ");

        assert_eq!(2, parse(s, "test").len());
    }

    #[test]
    fn parse_mixed_flashcard_types()
    {
        #[rustfmt::skip]
        let s = Cursor::new("
        # WriteTheLine
            this is a write-the-line flashcard
        
        ## FillTheBlank
            this is a FILL_THE_BLANK flashcard
        ");

        let flashcards = parse(s, "test");
        println!("{:?}", flashcards);
        assert_eq!(2, flashcards.len());
    }
}
