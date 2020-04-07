//! A module for different types of flashcards.

use super::*;

use crate::constants::BLANK_INDICATOR;
use crate::display::Color;
use crate::display::Display;
use crate::validator::{FlashcardValidator, InputValidator};

use std::hash::{Hash, Hasher};
use std::rc::Rc;

use twox_hash::XxHash64;

/// Represents a single flashcard.
#[derive(Clone, Debug)]
pub struct Flashcard
{
    /// The subject this flashcard belongs to.
    pub subject: Rc<String>,

    /// The front of a flashcard.
    pub face: Line,

    /// The back of a flashcard, which can have different types.
    pub back: FlashcardBack,

    /// An optional note on the flashcard to provide additional context.
    pub note: Option<Line>,
}

impl Flashcard
{
    /// Returns the hash of this flashcard.
    ///
    /// This allows us to store progress across changing/fixing flashcard fronts and
    /// adding notes.
    pub fn get_hash(&self) -> u64
    {
        use FlashcardBack::*;

        let mut hasher = XxHash64::default();
        hasher.write(self.subject.as_bytes());
        match &self.back {
            FillTheBlank(lines) => {
                for line in lines {
                    for line_part in line {
                        hasher.write(line_part.0.as_bytes());
                    }
                }
            }
            WriteTheLine(lines) => {
                for line in lines {
                    hasher.write(line.as_bytes());
                }
            }
        }
        hasher.finish()
    }

    /// For all flashcard types returns the characters per line to validate.
    ///
    /// All flashcards, no matter their type, require the user to enter something which
    /// needs to be checked against the expected.
    pub fn get_lines_to_validate(&self) -> Vec<Line>
    {
        use FlashcardBack::*;

        match &self.back {
            WriteTheLine(lines) => {
                // all lines needs to be validated
                lines.clone()
            }
            FillTheBlank(lines) => {
                // only blanks need to be validated
                let mut result = vec![];
                for line in lines {
                    let mut s = String::new();
                    for line_part in line {
                        // `true` means, that this part has to entered by the user
                        if line_part.1 {
                            s.push_str(&line_part.0);
                        }
                    }
                    result.push(s);
                }
                result
            }
        }
    }

    // This doesn't work because displaying is different for each flashcard type:
    // - FillTheBlanks: blanks are displayed with underscores.
    // - WriteTheLine: no underscores
    /// TODO: remove this function.
    pub fn get_lines_to_display(&self) -> Vec<Line>
    {
        use FlashcardBack::*;
        let mut result = vec![];

        match &self.back {
            WriteTheLine(lines) => {
                // no context provided
                for _ in lines {
                    result.push(String::new());
                }
                result
            }
            FillTheBlank(lines) => {
                // only the blanks will be validated
                for line in lines {
                    let mut s = String::new();
                    for line_part in line {
                        if line_part.1 {
                            // replace with blanks " "
                            s.push_str(
                                &line_part
                                    .0
                                    .chars()
                                    .map(|_| BLANK_INDICATOR)
                                    .collect::<String>(),
                            );
                        } else {
                            s.push_str(&line_part.0);
                        }
                        s.push_str(" ");
                    }
                    result.push(s);
                }
                result
            }
        }
    }

    /// TEMP: creates a validator for the whole card.
    pub fn get_validator(&self) -> FlashcardValidator
    {
        FlashcardValidator::new(self)
    }

    /// Displays the front of the flashcard.
    pub fn display_face(&self, display: &mut Display)
    {
        display.println_cr(format!("{}", self.face));
    }

    /// Displays the note on the flashcard.
    pub fn display_note(&self, display: &mut Display)
    {
        if let Some(note) = &self.note {
            display.cprintln_cr(format!("({})\n", note), Color::Yellow);
        }
    }
}

impl Eq for Flashcard {}
impl PartialEq for Flashcard
{
    fn eq(&self, other: &Self) -> bool
    {
        self.get_hash() == other.get_hash()
    }
}
impl Hash for Flashcard
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        use FlashcardBack::*;
        self.subject.as_bytes().hash(state);
        match &self.back {
            FillTheBlank(lines) => {
                for line in lines {
                    for line_part in line {
                        line_part.0.as_bytes().hash(state);
                    }
                }
            }
            WriteTheLine(lines) => {
                for line in lines {
                    line.as_bytes().hash(state);
                }
            }
        }
    }
}

impl Iterator for Flashcard
{
    type Item = InputValidator;

    fn next(&mut self) -> Option<Self::Item>
    {
        // Iterating a flashcard means iterating its back.
        // .0 => an input validator for a single line
        // .1 =>
        unimplemented!()
    }
}
