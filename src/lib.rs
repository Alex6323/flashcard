//! flash - A flashcard inspired learning application for the terminal using IOTA
//! microtransactions for gamification.

#![deny(missing_docs, bad_style, unsafe_code)]

mod constants;
mod parser;

/// Represents a box of flashcards.
pub struct CardBox {
    flashcards: Vec<FlashCard>,
    size: usize,
}

/// Represents a single flashcard.
pub struct FlashCard {
    face: String,
    back: String,
}


impl CardBox {
    /// Creates a new cardbox.
    ///
    /// # Example
    /// ```
    /// # use flash::CardBox;
    /// let cardbox = CardBox::from_file("./sample_box.txt");
    /// ```
    pub fn from_file(fname: &str) -> Self {
        let flashcards = parser::parse(fname);
        let size = flashcards.len();

        Self { flashcards, size }
    }

    /// Creates a new learning session.
    pub fn start_session(&self) {}

    /// Returns the number of cards in the card box.
    pub fn size(&self) -> usize {
        self.size
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
