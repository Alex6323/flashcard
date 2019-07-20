//! This module holds the CardBox and the Flashcard abstractions, which are used
//! to test the user.

use crate::parser;

/// Represents a single flashcard.
#[derive(Clone, Debug)]
pub struct FlashCard {
    /// The front of a flashcard.
    pub face: String,
    /// The back of a flachard.
    pub back: String,
}

/// Represents a box of flashcards.
pub struct CardBox {
    flashcards: Vec<FlashCard>,
    index: usize,
    size: usize,
}

impl CardBox {
    /// Creates a new cardbox.
    ///
    /// # Example
    /// ```
    /// # use flash::prelude::*;
    /// let cardbox = CardBox::from_file("./sample_box.txt");
    /// ```
    pub fn from_file(path: &str) -> Self {
        let flashcards = parser::parse(path);
        let index = 0;
        let size = flashcards.len();

        Self { flashcards, index, size }
    }

    /// Creates a new learning session.
    pub fn start_session(&self) {}

    /// Returns the number of cards in the card box.
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Iterator for CardBox {
    type Item = FlashCard;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index < self.size {
            self.index += 1;
            Some(self.flashcards[index].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_cardbox() {
        let cardbox = CardBox::from_file("./sample_box.txt");

        assert_eq!(10, cardbox.size())
    }

    #[test]
    fn start_session() {
        let cardbox = CardBox::from_file("./sample_box.txt");
        cardbox.start_session();
    }
}
