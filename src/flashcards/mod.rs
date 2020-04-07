//! A module to represent and build flashcard.

pub mod flashcard;
pub mod flashcard_factory;

pub use self::flashcard::Flashcard;

/// Represents a single line on a flashcard.
pub type Line = String;

/// Represents a part of a line, and whether it is considered a blank, or not.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LinePart(pub String, pub bool, pub usize);

/// Represents a line that can contain blanks.
pub type LineWithBlanks = Vec<LinePart>;

/// Represents the various types of flashcard backs.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FlashcardBack
{
    /// Requires the user to fill in the blanks.
    FillTheBlank(Vec<LineWithBlanks>),

    /// Requires the user to write whole lines.
    WriteTheLine(Vec<Line>),
}
