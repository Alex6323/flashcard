//! Input validator
//!
//! Compares two strings
//! - the string on the back of a flashcard
//! - the string entered by the user

/// The Input validator
pub struct InputValidator {
    expected: Vec<char>,
    received: Vec<bool>,
    index: usize,
    length: usize,
}

impl InputValidator {
    /// Creates a new `InputValidator` for an expected String.
    pub fn new(expected: &str) -> Self {
        let expected = expected.chars().collect::<Vec<_>>();
        let length = expected.len();
        let received = vec![false; length];

        Self { expected, received, index: 0, length }
    }

    /// Checks the given character against the corresponding character of the expected
    /// string, and increases the index.
    pub fn check(&mut self, c: char) -> bool {
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
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// Undoes the last validation step.
    pub fn undo(&mut self) {
        if self.index == 0 {
            return;
        }
        self.index -= 1;
    }

    /// Returns true if the user has correctly entered all characters.
    pub fn is_happy(&self) -> bool {
        self.received.iter().all(|r| *r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_correct_input() {
        let mut v = InputValidator::new("hello");
        assert!(v.check('h'));
        assert!(v.check('e'));
        assert!(v.check('l'));
        assert!(v.check('l'));
        assert!(v.check('o'));
    }

    #[test]
    fn check_incorrect_input() {
        let mut v = InputValidator::new("hello");
        assert!(v.check('h'));
        assert!(!v.check('3'));
        assert!(v.check('l'));
        assert!(v.check('l'));
        assert!(!v.check('0'));
    }

    #[test]
    fn check_corrected_input() {
        let mut v = InputValidator::new("hello");
        assert!(v.check('h'));
        assert!(!v.check('3'));
        v.undo();
        assert!(v.check('e'));
        assert!(v.check('l'));
        assert!(v.check('l'));
        assert!(!v.check('0'));
        v.undo();
        assert!(v.check('o'));
    }

    #[test]
    fn check_validator_reset() {
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
    fn is_happy() {
        let mut v = InputValidator::new("hello");
        assert!(!v.is_happy());
        assert!(v.check('h'));
        assert!(!v.is_happy());
        assert!(v.check('e'));
        assert!(!v.is_happy());
        assert!(v.check('l'));
        assert!(!v.is_happy());
        assert!(v.check('l'));
        assert!(!v.is_happy());
        assert!(v.check('o'));
        assert!(v.is_happy());
    }

    #[test]
    fn is_happy_with_corrected_input() {
        let mut v = InputValidator::new("hello");
        assert!(!v.is_happy());
        assert!(v.check('h'));
        assert!(!v.check('3'));
        assert!(!v.is_happy());
        v.undo();
        assert!(!v.is_happy());
        assert!(v.check('e'));
        assert!(v.check('l'));
        assert!(!v.is_happy());
        assert!(v.check('l'));
        assert!(!v.check('0'));
        assert!(!v.is_happy());
        v.undo();
        assert!(!v.is_happy());
        assert!(v.check('o'));
        assert!(v.is_happy());
    }
}
