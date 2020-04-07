//! Module that hosts the cardbox responsible for flashcard selection.

use crate::cardbox_parser;
use crate::common::fs;
use crate::common::time;
use crate::constants::INITIAL_QUEUE_SIZE;
use crate::db::{self, Stage};
use crate::display::Display;
use crate::flashcards::Flashcard;

use std::collections::{HashMap, VecDeque};

/// Represents a flashcard with additional metadata.
#[derive(Debug)]
pub struct Envelope
{
    /// A pointer to the associated flashcard.
    pub flashcard: Flashcard,
    /// The hash of the flashcard.
    pub hash: u64,
    /// The time the flashcard entered its current stage (Unix time in milliseconds).
    pub timestamp: u64,
}

/// Represents the current learning progress.
pub struct Progress(pub usize, pub usize, pub usize, pub usize, pub usize, pub usize);

/// Represents a cardbox that by some logic deals the flashcards based on the user's
/// learning progress.
pub struct Cardbox
{
    stage0: VecDeque<Flashcard>,
    stage1: VecDeque<Envelope>,
    stage2: VecDeque<Envelope>,
    stage3: VecDeque<Envelope>,
    stage4: VecDeque<Envelope>,
    stage5: VecDeque<Envelope>,
    progress: HashMap<u64, Stage>,
}

impl Cardbox
{
    /// Creates a new flashcard cardbox.
    pub fn new() -> Self
    {
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

    /// Tries to put the flashcard into the cardbox.
    ///
    /// This will only succeed, if the card was previously taken from the cardbox and put
    /// into the cardbox.
    pub fn init(&mut self, path: &str)
    {
        let flashcards = cardbox_parser::parse_from_file(path);
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
    pub fn increase_stage(&mut self, current_stage: usize)
    {
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
    pub fn reset_stage(&mut self, current_stage: usize)
    {
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
    pub fn save(&mut self)
    {
        for envelope in self.stage1.iter() {
            let stage = Stage { index: 1, timestamp_ms: envelope.timestamp };
            self.progress.insert(envelope.hash, stage);
        }
        for envelope in self.stage2.iter() {
            let stage = Stage { index: 2, timestamp_ms: envelope.timestamp };
            self.progress.insert(envelope.hash, stage);
        }
        for envelope in self.stage3.iter() {
            let stage = Stage { index: 3, timestamp_ms: envelope.timestamp };
            self.progress.insert(envelope.hash, stage);
        }
        for envelope in self.stage4.iter() {
            let stage = Stage { index: 4, timestamp_ms: envelope.timestamp };
            self.progress.insert(envelope.hash, stage);
        }
        for envelope in self.stage5.iter() {
            let stage = Stage { index: 5, timestamp_ms: envelope.timestamp };
            self.progress.insert(envelope.hash, stage);
        }
        db::save(&self.progress, &fs::get_progress_db_path())
            .expect("error saving database");
    }

    /// Returns the number of flashcards currently being actively processed.
    pub fn num_active(&self) -> usize
    {
        self.stage1.len()
            + self.stage2.len()
            + self.stage3.len()
            + self.stage4.len()
            + self.stage5.len()
    }

    /// Returns the number of all flashcards in the cardbox.
    pub fn size(&self) -> usize
    {
        self.stage0.len()
            + self.stage1.len()
            + self.stage2.len()
            + self.stage3.len()
            + self.stage4.len()
            + self.stage5.len()
    }

    /// Returns the next flashcard and its current stage.
    pub fn next(&self) -> Option<(&Flashcard, usize)>
    {
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

    /// Returns the current progress.
    ///
    /// This measured by simply counting the flashcards of each stage.
    /// TODO: remove this methods.
    pub fn progress(&self) -> Progress
    {
        Progress(
            self.stage0.len(),
            self.stage1.len(),
            self.stage2.len(),
            self.stage3.len(),
            self.stage4.len(),
            self.stage5.len(),
        )
    }

    /// Displays the current progress.
    pub fn display_progress(&self, display: &mut Display)
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn new_cardbox()
    {
        let cardbox = Cardbox::new();
        assert_eq!(0, cardbox.stage0.len());
        assert_eq!(0, cardbox.stage1.len());
    }

    #[test]
    fn init_cardbox()
    {
        let mut cardbox = Cardbox::new();

        cardbox.init("./sample_box.txt");
        assert_eq!(32, cardbox.size());
        assert_eq!(20, cardbox.num_active());
        assert_eq!(12, cardbox.stage0.len());
        assert_eq!(10, cardbox.stage1.len());
        assert_eq!(2, cardbox.stage2.len());
        assert_eq!(4, cardbox.stage3.len());
        assert_eq!(3, cardbox.stage4.len());
        assert_eq!(1, cardbox.stage5.len());
    }
}
