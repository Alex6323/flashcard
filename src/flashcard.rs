//! This module holds the Flashcard abstractions, which are used
//! to test the user.

use std::hash::{Hash, Hasher};
use std::rc::Rc;

use twox_hash::XxHash64;

/// Represents a line on a flashcard.
pub type Line = String;

/// Represents a list of lines on a flashcard.
pub type List = Vec<Line>;

/// Represents a single flashcard.
#[derive(Clone, Debug)]
pub struct FlashCard {
    /// Meta information about this flashcard (e.g. the subject)
    pub subject: Rc<String>,

    /// The front of a flashcard.
    pub face: Line,

    /// The back of a flashcard, which can consist of several lines.
    pub back: List,

    /// An optional note on the flashcard to provide helpful context.
    pub note: Option<Line>,
}

impl FlashCard {
    /// Returns the hash of this flashcard.
    ///
    /// The hash of a `FlashCard` is determined by:
    /// - the subject it belongs to,
    /// - the information on the back of the flashcard.
    ///
    /// This allows us to store progress across changing/fixing flashcard fronts and
    /// adding notes.
    pub fn get_hash(&self) -> u64 {
        let mut hasher = XxHash64::default();
        hasher.write(self.subject.as_bytes());
        for item in &self.back {
            hasher.write(item.as_bytes());
        }
        hasher.finish()
    }
}

impl Eq for FlashCard {}
impl PartialEq for FlashCard {
    fn eq(&self, other: &Self) -> bool {
        self.get_hash() == other.get_hash()
    }
}
impl Hash for FlashCard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.subject.as_bytes().hash(state);
        for item in &self.back {
            item.as_bytes().hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::time;
    use super::*;

    #[test]
    fn hash_flashcard() {
        let subject1 = Rc::new(String::from("subject1"));
        let subject2 = Rc::new(String::from("subject2"));

        let fc1 = FlashCard {
            subject: Rc::clone(&subject1),
            face: String::from("hello"),
            back: vec![String::from("world")],
            note: Some(String::new()),
        };
        let fc2 = FlashCard {
            subject: Rc::clone(&subject1),
            face: String::from("hi"),
            back: vec![String::from("world")],
            note: Some(String::from("a note")),
        };
        assert_eq!(fc1.get_hash(), fc2.get_hash());

        let fc3 = FlashCard {
            subject: Rc::clone(&subject2),
            face: String::from("hello"),
            back: vec![String::from("world")],
            note: Some(String::new()),
        };
        assert_ne!(fc1.get_hash(), fc3.get_hash());
    }

}
