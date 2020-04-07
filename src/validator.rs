//! A module for providing input validators.
//!
//! Essentially compares two strings whether they match exactly:
//! - the string on the back of a flashcard (expected)
//! - the string entered by the user (received)
//!
//! Usecases:
//! - programming syntax
//! - vocabulary
//! - key settings
//!
//! # Example
//! ```
//! # use flashcard::validator::InputValidator;
//!
//! let mut v = InputValidator::new("hello world!");
//!
//! assert_eq!(12, v.length());
//! ```

use crate::flashcards::Flashcard;

/// TODO: rename to PeekMode
#[derive(Clone)]
pub enum HintMode
{
    /// No hints are displayed.
    Inactive,
    /// A certain number of characters are shown.
    Active(usize),
}

///
#[derive(Clone)]
pub struct InputValidator
{
    /// The list of expected characters.
    expected: Vec<char>,

    //displayed: String,
    /// For each character a flag whether it matches the expected character.
    received: Vec<bool>,

    /// The current index the validator is pointing at.
    index: usize,

    /// The number of characters of the expected input the validator is checking against.
    length: usize,

    /// Indicates whether hint mode is currently active.
    hint_mode: HintMode,

    /// Wether the received characters all match the expected characters.
    passed: bool,
}

impl InputValidator
{
    /// Creates a new instance from the `expected` characters.
    ///
    /// TODO: semantically an InputValidator shouldn't have to care about what is
    /// displayed, so remove that.
    pub fn new(expected: &str) -> Self
//pub fn new(expected: &str, displayed: &str) -> Self
    {
        let expected = expected.chars().collect::<Vec<_>>();
        let length = expected.len();
        let received = vec![false; length];
        let passed = true;

        Self {
            expected,
            //displayed: String::from(displayed),
            received,
            index: 0,
            length,
            hint_mode: HintMode::Inactive,
            passed,
        }
    }
}

impl InputValidator
{
    /// Checks the given character against the corresponding character of the expected
    /// string, and increases the index.
    pub fn check(&mut self, c: char) -> bool
    {
        if self.index >= self.length {
            return false;
        }
        let index = self.index;
        self.index += 1;

        let is_valid = self.expected[index] == c;
        self.received[index] = is_valid;

        is_valid
    }

    /// Resets this validator.
    pub fn reset(&mut self)
    {
        self.index = 0;
        self.received.iter_mut().for_each(|r| *r = false);
    }

    /// Undoes the last number of validation steps.
    pub fn undo(&mut self, num: usize)
    {
        if self.index == 0 {
            return;
        }

        for _ in 0..num {
            self.index -= 1;
            self.received[self.index] = false;
        }
    }

    /// Returns `true` if the user has correctly entered all characters.
    pub fn happy(&self) -> bool
    {
        self.received.iter().all(|r| *r)
    }

    /// Returns `true` if the validator is still accepting more characters.
    pub fn accepts(&self) -> bool
    {
        self.index < self.length
    }

    /// Activates the hint mode, and returns a hint/part of the flashcard back.
    ///
    /// Repeated calls will reveal more information until there is either nothing to
    /// reveal or the `hint_close` call.
    pub fn peek(&mut self) -> Option<char>
    {
        match self.hint_mode {
            HintMode::Inactive => {
                self.hint_mode = HintMode::Active(self.index);
                return Some(self.expected[self.index]);
            }
            HintMode::Active(index) => {
                self.passed = false;
                if index < self.length - 1 {
                    self.hint_mode = HintMode::Active(index + 1);
                    return Some(self.expected[index + 1]);
                }
            }
        }
        None
    }

    /// Ends the hint mode.
    pub fn end_peek(&mut self)
    {
        self.hint_mode = HintMode::Inactive;
    }

    /// Returns the index of the first incorrect character, or None if such a character
    /// doesn't exist.
    pub fn first_incorrect(&self) -> Option<usize>
    {
        self.received.iter().position(|r| !*r)
    }

    /// Returns the expected `String`.
    pub fn expected(&self) -> String
    {
        self.expected.iter().collect::<String>()
    }

    /// Returns the current character index.
    pub fn index(&self) -> usize
    {
        self.index
    }

    /// Returns the current hint mode.
    pub fn hint_mode(&self) -> &HintMode
    {
        &self.hint_mode
    }

    /// Returns the number of characters.
    pub fn length(&self) -> usize
    {
        self.length
    }

    /// Returns `true` if the user entered the line correctly within the threshold allowed
    /// mistakes.
    pub fn passed(&self) -> bool
    {
        self.passed
    }

    /*
    pub fn context(&self) -> String
    {
        self.displayed.clone()
    }
    */
}

/// Represents a validator for the whole flashcard, and is comprised of several input
/// validators.
pub struct FlashcardValidator
{
    /// A list of input validators for each line.
    pub validators: Vec<InputValidator>,
    index: usize,
    length: usize,
}

impl FlashcardValidator
{
    /// Creates a new instance.
    //pub fn new(list: &Vec<String>) -> Self {
    pub fn new(flashcard: &Flashcard) -> Self
    {
        //let lines_to_display = flashcard.get_lines_to_display();
        let lines_to_validate = flashcard.get_lines_to_validate();

        let length = lines_to_validate.len();

        let mut validators = Vec::with_capacity(length);
        /*
        for (validate, display) in lines_to_validate.iter().zip(lines_to_display) {
            validators.push(InputValidator::new(&validate, &display));
        }
        */
        for validate in lines_to_validate {
            validators.push(InputValidator::new(&validate));
        }

        Self { validators, index: 0, length }
    }

    /// Returns the index of the currently active input validator.
    pub fn index(&self) -> usize
    {
        self.index
    }

    /// Returns the number of contained input validators.
    pub fn length(&self) -> usize
    {
        self.length
    }

    /// Returns `true` if all sub-validators are "happy".
    pub fn happy(&self) -> bool
    {
        self.validators.iter().all(|v| v.happy())
    }

    /// Returns `true` if all sub-validators were passed.
    pub fn passed(&self) -> bool
    {
        self.validators.iter().all(|v| v.passed)
    }
}

/*
impl Iterator for FlashcardValidator
{
    type Item = InputValidator;
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.length {
            let i = self.index;
            self.index += 1;
            Some(self.validators[i].clone())
        } else {
            None
        }
    }
}
*/

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn check_correct_input()
    {
        let mut v = InputValidator::new("hello");
        assert!(v.check('h'));
        assert!(v.check('e'));
        assert!(v.check('l'));
        assert!(v.check('l'));
        assert!(v.check('o'));
    }

    #[test]
    fn check_incorrect_input()
    {
        let mut v = InputValidator::new("hello");
        assert!(v.check('h'));
        assert!(!v.check('3'));
        assert!(v.check('l'));
        assert!(v.check('l'));
        assert!(!v.check('0'));
    }

    #[test]
    fn undo_1()
    {
        let mut v = InputValidator::new("hello");
        v.check('h');
        v.check('3');
        v.undo(1);
        v.check('e');
        v.check('l');
        v.check('l');
        v.check('0');
        v.undo(1);
        v.check('o');

        assert!(v.happy());
    }

    #[test]
    fn undo_0()
    {
        let mut v = InputValidator::new("hello");
        v.check('h');
        v.undo(0);
        v.check('e');
        v.undo(0);
        v.check('l');
        v.undo(0);
        v.check('l');
        v.undo(0);
        v.check('o');
        v.undo(0);
        assert!(v.happy());
    }

    #[test]
    fn undo_many()
    {
        let mut v = InputValidator::new("hello");
        v.check('h');
        v.check('e');
        v.undo(2);
        v.check('h');
        v.check('e');
        v.check('l');
        v.check('l');
        v.check('o');
        assert!(v.happy());
    }

    #[test]
    fn check_validator_reset()
    {
        let mut v = InputValidator::new("hello");
        assert!(v.check('h'));
        assert!(v.check('e'));
        v.reset();
        assert!(v.check('h'));
        assert!(v.check('e'));
        assert!(v.check('l'));
        assert!(v.check('l'));
        assert!(v.check('o'));
    }

    #[test]
    fn is_happy()
    {
        let mut v = InputValidator::new("hello");
        assert!(!v.happy());
        assert!(v.check('h'));
        assert!(!v.happy());
        assert!(v.check('e'));
        assert!(!v.happy());
        assert!(v.check('l'));
        assert!(!v.happy());
        assert!(v.check('l'));
        assert!(!v.happy());
        assert!(v.check('o'));
        assert!(v.happy());
    }

    #[test]
    fn is_happy_with_corrected_input()
    {
        let mut v = InputValidator::new("hello");
        assert!(!v.happy());
        assert!(v.check('h'));
        assert!(!v.check('3'));
        assert!(!v.happy());
        v.undo(1);
        assert!(!v.happy());
        assert!(v.check('e'));
        assert!(v.check('l'));
        assert!(!v.happy());
        assert!(v.check('l'));
        assert!(!v.check('0'));
        assert!(!v.happy());
        v.undo(1);
        assert!(!v.happy());
        assert!(v.check('o'));
        assert!(v.happy());
    }

    #[test]
    fn hint()
    {
        let mut v = InputValidator::new("hello");
        assert_eq!(Some('h'), v.peek());
        assert_eq!(Some('e'), v.peek());
    }

    #[test]
    fn hints_dont_satisfy_validator()
    {
        let mut v = InputValidator::new("hello");
        assert_eq!(Some('h'), v.peek());
        assert_eq!(Some('e'), v.peek());
        assert_eq!(Some('l'), v.peek());
        assert_eq!(Some('l'), v.peek());
        assert_eq!(Some('o'), v.peek());
        assert!(!v.happy());
    }

    #[test]
    fn none_incorrect()
    {
        let mut v = InputValidator::new("hello");

        assert_eq!(Some(0), v.first_incorrect());

        v.check('h');
        v.check('e');
        v.check('l');
        v.check('l');
        v.check('o');

        assert_eq!(None, v.first_incorrect());
    }

    #[test]
    fn first_incorrect()
    {
        let mut v = InputValidator::new("hello");

        assert_eq!(Some(0), v.first_incorrect());

        v.check('h');
        v.check('e');

        // NOTE: per default missing characters are considered incorrect
        assert_eq!(Some(2), v.first_incorrect());

        v.check('l');
        v.check('1');
        v.check('o');

        assert_eq!(Some(3), v.first_incorrect());
    }

    #[test]
    fn get_expected()
    {
        let v = InputValidator::new("hello");
        assert_eq!("hello", &v.expected());
    }

    #[test]
    fn list_validator_is_happy()
    {
        let mut v = FlashcardValidator::new(&vec!["hello".into(), "world".into()]);
        v.validators[0].check('h');
        v.validators[0].check('e');
        v.validators[0].check('l');
        v.validators[0].check('l');
        v.validators[0].check('o');

        v.validators[1].check('w');
        v.validators[1].check('o');
        v.validators[1].check('r');
        v.validators[1].check('l');
        v.validators[1].check('d');

        v.happy();
    }
}
