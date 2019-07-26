//! Module that hosts the automat responsible for flashcard selection.

use crate::common::fs;
use crate::common::time;
use crate::constants::{DB_NAME, INITIAL_QUEUE_SIZE};
use crate::db::{self, Stage};
use crate::flashcard::FlashCard;
use crate::parser;

use std::collections::{HashMap, VecDeque};

/// Represents a flashcard with additional metadata.
#[derive(Debug)]
pub struct Envelope {
    /// A pointer to the associated flashcard.
    pub flashcard: FlashCard,
    /// The hash of the flashcard.
    pub hash: u64,
    /// The time the flashcard entered its current stage (Unix time in milliseconds).
    pub timestamp: u64,
}

/// Represents an automat that by some logic deals the flashcards based on the user's
/// learning progress.
pub struct Automat {
    stage0: VecDeque<FlashCard>,
    stage1: VecDeque<Envelope>,
    stage2: VecDeque<Envelope>,
    stage3: VecDeque<Envelope>,
    stage4: VecDeque<Envelope>,
    stage5: VecDeque<Envelope>,
    progress: HashMap<u64, Stage>,
}

impl Automat {
    /// Creates a new flashcard automat.
    pub fn new() -> Self {
        Self {
            stage0: VecDeque::new(),
            stage1: VecDeque::new(),
            stage2: VecDeque::new(),
            stage3: VecDeque::new(),
            stage4: VecDeque::new(),
            stage5: VecDeque::new(),
            progress: db::load(&fs::get_progress_db_path()),
        }
    }

    /// Tries to put the flashcard into the automat.
    ///
    /// This will only succeed, if the card was previously taken from the cardbox and put
    /// into the automat.
    pub fn init(&mut self, path: &str) {
        let flashcards = parser::parse(path);
        // Fill all stages according to the progress database
        for flashcard in flashcards.into_iter() {
            let hash = flashcard.get_hash();

            if let Some(stage) = self.progress.get(&hash) {
                let timestamp = stage.timestamp_ms;

                match stage.index {
                    1 => self.stage1.push_back(Envelope { flashcard, hash, timestamp }),
                    2 => self.stage2.push_back(Envelope { flashcard, hash, timestamp }),
                    3 => self.stage3.push_back(Envelope { flashcard, hash, timestamp }),
                    4 => self.stage4.push_back(Envelope { flashcard, hash, timestamp }),
                    5 => self.stage5.push_back(Envelope { flashcard, hash, timestamp }),
                    _ => panic!("error: invalid stage in progress database"),
                }
            } else {
                self.stage0.push_back(flashcard);
            }
        }

        // If there is still room in Stage 1, then fill it with flashcards from the
        // cardbox
        while !self.stage0.is_empty() && self.stage1.len() < INITIAL_QUEUE_SIZE {
            let flashcard = self.stage0.pop_front().unwrap(); // cannot fail
            let hash = flashcard.get_hash();
            let timestamp = time::get_unix_time_millis();

            self.stage1.push_back(Envelope { flashcard, hash, timestamp });
        }
    }

    /// Increases the stage of the flashcard.
    pub fn increase_stage(&mut self, current_stage: usize) {
        match current_stage {
            5 => {
                // let flashcards stay in the last stage forever
                let mut envelope = self.stage5.pop_front().unwrap();
                envelope.timestamp = time::get_unix_time_millis();
                self.stage5.push_back(envelope);
            }
            4 => {
                let mut envelope = self.stage4.pop_front().unwrap();
                envelope.timestamp = time::get_unix_time_millis();
                self.stage5.push_back(envelope);
            }
            3 => {
                let mut envelope = self.stage3.pop_front().unwrap();
                envelope.timestamp = time::get_unix_time_millis();
                self.stage4.push_back(envelope);
            }
            2 => {
                let mut envelope = self.stage2.pop_front().unwrap();
                envelope.timestamp = time::get_unix_time_millis();
                self.stage3.push_back(envelope);
            }
            1 => {
                let mut envelope = self.stage1.pop_front().unwrap();
                envelope.timestamp = time::get_unix_time_millis();
                self.stage2.push_back(envelope);

                // refill stage 1 if necessary
                while self.stage1.len() < INITIAL_QUEUE_SIZE && !self.stage0.is_empty() {
                    let flashcard = self.stage0.pop_front().unwrap();
                    let hash = flashcard.get_hash();
                    let timestamp = time::get_unix_time_millis();
                    let envelope = Envelope { flashcard, hash, timestamp };
                    self.stage1.push_back(envelope);
                }
            }
            _ => panic!("error: invalid stage"),
        }
    }

    /// Resets the stage of the flashcard.
    pub fn reset_stage(&mut self, current_stage: usize) {
        let mut envelope = match current_stage {
            5 => self.stage5.pop_front().unwrap(),
            4 => self.stage4.pop_front().unwrap(),
            3 => self.stage3.pop_front().unwrap(),
            2 => self.stage2.pop_front().unwrap(),
            1 => self.stage1.pop_front().unwrap(),
            _ => panic!("error: invalid stage"),
        };
        envelope.timestamp = time::get_unix_time_millis();
        self.stage1.push_back(envelope);
    }

    /// Saves the progress to the internal key-value store.
    pub fn save(&self) {
        db::save(&self.progress, DB_NAME).expect("error saving database");
    }

    /// Returns the number of flashcards currently processed by this automat.
    pub fn size(&self) -> usize {
        self.stage1.len()
            + self.stage2.len()
            + self.stage3.len()
            + self.stage4.len()
            + self.stage5.len()
    }

    /// Returns the next flashcard and its current stage.
    pub fn next(&self) -> Option<(&FlashCard, usize)> {
        use crate::constants::STAGE1_COOLDOWN;
        use crate::constants::STAGE2_COOLDOWN;
        use crate::constants::STAGE3_COOLDOWN;
        use crate::constants::STAGE4_COOLDOWN;
        use crate::constants::STAGE5_COOLDOWN;

        let current_time = time::get_unix_time_millis();

        if let Some(envelope) = self.stage5.front() {
            if envelope.timestamp <= (current_time - STAGE5_COOLDOWN * 1000) {
                return Some((&envelope.flashcard, 5));
            }
        }

        if let Some(envelope) = self.stage4.front() {
            if envelope.timestamp <= (current_time - STAGE4_COOLDOWN * 1000) {
                return Some((&envelope.flashcard, 4));
            }
        }

        if let Some(envelope) = self.stage3.front() {
            if envelope.timestamp <= (current_time - STAGE3_COOLDOWN * 1000) {
                return Some((&envelope.flashcard, 3));
            }
        }

        if let Some(envelope) = self.stage2.front() {
            if envelope.timestamp <= (current_time - STAGE2_COOLDOWN * 1000) {
                return Some((&envelope.flashcard, 2));
            }
        }
        if let Some(envelope) = self.stage1.front() {
            if envelope.timestamp <= (current_time - STAGE1_COOLDOWN * 1000) {
                return Some((&envelope.flashcard, 1));
            }
        }

        // only ends if there are no flashcards in the first queue and all
        // other flashcards need to pause
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_automat() {
        let mut automat = Automat::new();
        assert_eq!(0, automat.stage0.len());
        assert_eq!(0, automat.stage1.len());
    }

    #[test]
    fn init_automat() {
        let mut automat = Automat::new();

        automat.init("./sample_box.txt");
        assert_eq!(20, automat.size());
        assert_eq!(11, automat.stage0.len());
        assert_eq!(10, automat.stage1.len());
        assert_eq!(2, automat.stage2.len());
        assert_eq!(4, automat.stage3.len());
        assert_eq!(3, automat.stage4.len());
        assert_eq!(1, automat.stage5.len());
    }
}
