//! List validator
//! - Consists of several `Line Validators`.
//!
//! Line validator
//!
//! Compares two strings
//! - the string on the back of a flashcard
//! - the string entered by the user

/// Represents a validator for a list of lines.
///
/// The whole list is validated once all sub-validators are "happy".
pub struct ListValidator {
    /// The contained list of `Line Validators`.
    pub validators: Vec<LineValidator>,
    ///
    pub index: usize,
    ///
    pub length: usize,
}

impl ListValidator {
    /// Creates a new `ListValidator`.
    pub fn new(list: Vec<String>) -> Self {
        let mut validators = Vec::with_capacity(list.len());

        for s in list {
            validators.push(LineValidator::new(&s));
        }

        let length = validators.len();

        Self { validators, index: 0, length }
    }

    /// Returns `true` if all sub-validators are "happy".
    pub fn is_happy(&self) -> bool {
        self.validators.iter().all(|v| v.is_happy())
    }
}

impl Iterator for ListValidator {
    type Item = LineValidator;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.length {
            let i = self.index;
            self.index += 1;
            Some(self.validators[i].clone())
        } else {
            None
        }
    }
}

/// Represents a validator for a single line of text.
#[derive(Clone)]
pub struct LineValidator {
    expected: Vec<char>,
    received: Vec<bool>,
    /// The current index the validator is pointing at.
    pub index: usize,
    /// The number of characters of the expected input the validator is checking against.
    pub length: usize,
    /// Indicates whether hint mode is currently active.
    pub hint_mode: HintMode,
}

#[derive(Clone)]
pub enum HintMode {
    Inactive,
    Active(usize),
}

impl LineValidator {
    /// Creates a new `LineValidator` for a `String` to check user input against.
    pub fn new(expected: &str) -> Self {
        let expected = expected.chars().collect::<Vec<_>>();
        let length = expected.len();
        let received = vec![false; length];

        Self { expected, received, index: 0, length, hint_mode: HintMode::Inactive }
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
        self.received.iter_mut().for_each(|r| *r = false);
    }

    /// Undoes the last validation step.
    pub fn undo(&mut self, num: usize) {
        if self.index == 0 {
            return;
        }

        for _ in 0..num {
            self.index -= 1;
            self.received[self.index] = false;
        }
    }

    /// Returns `true` if the user has correctly entered all characters.
    pub fn is_happy(&self) -> bool {
        self.received.iter().all(|r| *r)
    }

    /// Returns `true` if the validator is still accepting more characters.
    pub fn is_accepting(&self) -> bool {
        self.index < self.length
    }

    /// Activates the hint mode, and returns a hint/part of the flashcard back.
    ///
    /// Repeated calls will reveal more information until there is either nothing to
    /// reveal or the `hint_close` call.
    pub fn hint(&mut self) -> Option<char> {
        match self.hint_mode {
            HintMode::Inactive => {
                self.hint_mode = HintMode::Active(self.index);
                return Some(self.expected[self.index]);
            }
            HintMode::Active(index) => {
                if index < self.length - 1 {
                    self.hint_mode = HintMode::Active(index + 1);
                    return Some(self.expected[index + 1]);
                }
            }
        }
        None
    }

    /// Ends the hint mode.
    pub fn hint_close(&mut self) {
        self.hint_mode = HintMode::Inactive;
    }

    /// Returns the index of the first incorrect character, or None if such a character
    /// doesn't exist.
    pub fn first_incorrect(&self) -> Option<usize> {
        self.received.iter().position(|r| !*r)
    }

    /// Returns the expected `String`.
    pub fn get_expected(&self) -> String {
        self.expected.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_correct_input() {
        let mut v = LineValidator::new("hello");
        assert!(v.check('h'));
        assert!(v.check('e'));
        assert!(v.check('l'));
        assert!(v.check('l'));
        assert!(v.check('o'));
    }

    #[test]
    fn check_incorrect_input() {
        let mut v = LineValidator::new("hello");
        assert!(v.check('h'));
        assert!(!v.check('3'));
        assert!(v.check('l'));
        assert!(v.check('l'));
        assert!(!v.check('0'));
    }

    #[test]
    fn undo_1() {
        let mut v = LineValidator::new("hello");
        v.check('h');
        v.check('3');
        v.undo(1);
        v.check('e');
        v.check('l');
        v.check('l');
        v.check('0');
        v.undo(1);
        v.check('o');

        assert!(v.is_happy());
    }

    #[test]
    fn undo_0() {
        let mut v = LineValidator::new("hello");
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
        assert!(v.is_happy());
    }

    #[test]
    fn undo_many() {
        let mut v = LineValidator::new("hello");
        v.check('h');
        v.check('e');
        v.undo(2);
        v.check('h');
        v.check('e');
        v.check('l');
        v.check('l');
        v.check('o');
        assert!(v.is_happy());
    }

    #[test]
    fn check_validator_reset() {
        let mut v = LineValidator::new("hello");
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
        let mut v = LineValidator::new("hello");
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
        let mut v = LineValidator::new("hello");
        assert!(!v.is_happy());
        assert!(v.check('h'));
        assert!(!v.check('3'));
        assert!(!v.is_happy());
        v.undo(1);
        assert!(!v.is_happy());
        assert!(v.check('e'));
        assert!(v.check('l'));
        assert!(!v.is_happy());
        assert!(v.check('l'));
        assert!(!v.check('0'));
        assert!(!v.is_happy());
        v.undo(1);
        assert!(!v.is_happy());
        assert!(v.check('o'));
        assert!(v.is_happy());
    }

    #[test]
    fn hint() {
        let mut v = LineValidator::new("hello");
        assert_eq!(Some('h'), v.hint());
        assert_eq!(Some('e'), v.hint());
    }

    #[test]
    fn hints_dont_satisfy_validator() {
        let mut v = LineValidator::new("hello");
        assert_eq!(Some('h'), v.hint());
        assert_eq!(Some('e'), v.hint());
        assert_eq!(Some('l'), v.hint());
        assert_eq!(Some('l'), v.hint());
        assert_eq!(Some('o'), v.hint());
        assert!(!v.is_happy());
    }

    #[test]
    fn none_incorrect() {
        let mut v = LineValidator::new("hello");

        assert_eq!(Some(0), v.first_incorrect());

        v.check('h');
        v.check('e');
        v.check('l');
        v.check('l');
        v.check('o');

        assert_eq!(None, v.first_incorrect());
    }

    #[test]
    fn first_incorrect() {
        let mut v = LineValidator::new("hello");

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
    fn get_expected() {
        let mut v = LineValidator::new("hello");

        assert_eq!("hello", &v.get_expected());
    }

    #[test]
    fn list_validator_is_happy() {
        let mut lv = ListValidator::new(vec!["hello".into(), "world".into()]);
        lv.validators[0].check('h');
        lv.validators[0].check('e');
        lv.validators[0].check('l');
        lv.validators[0].check('l');
        lv.validators[0].check('o');

        lv.validators[1].check('w');
        lv.validators[1].check('o');
        lv.validators[1].check('r');
        lv.validators[1].check('l');
        lv.validators[1].check('d');

        lv.is_happy();
    }
}
