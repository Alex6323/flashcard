//! A module to represent flashcards that require the user to enter all characters of a
//! line.

use super::*;

use std::rc::Rc;

/// A factory to produce flashcards from markup.
pub struct FlashcardFactory
{
    subject: Rc<String>,
    face: Option<Line>,
    back: Option<FlashcardBack>,
    note: Option<Line>,
}

impl FlashcardFactory
{
    /// Creates a new flashcard factory.
    pub fn new(subject: &str) -> Self
    {
        Self {
            subject: Rc::new(String::from(subject)),
            face: None,
            back: None,
            note: None,
        }
    }

    /// Sets the front side of the flashcard.
    pub fn set_face(&mut self, face_text: &str)
    {
        self.face = Some(String::from(face_text));
    }

    /// Sets the back of the flashcard.
    pub fn set_back(&mut self, back: FlashcardBack)
    {
        self.back = Some(back);
    }

    /// Sets a note to the flashcard to provide helpful context.
    pub fn set_note(&mut self, note_text: &str)
    {
        self.note = Some(String::from(note_text));
    }

    /// Returns `true` if enough data has been provided to build a flashcard.
    pub fn can_build(&self) -> bool
    {
        self.face.is_some() && !self.back.is_none()
    }

    /// Builds a flashcard from the current state of the factory.
    pub fn build(&mut self) -> Flashcard
    {
        assert!(self.can_build());

        let face = std::mem::replace(&mut self.face, None).unwrap();
        let back = std::mem::replace(&mut self.back, None).unwrap();
        let note = std::mem::replace(&mut self.note, None);

        Flashcard { subject: Rc::clone(&self.subject), face, back, note }
    }
}

#[cfg(test)]
mod tests
{
    /*
    use super::super::super::common::time;
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
    */
}
