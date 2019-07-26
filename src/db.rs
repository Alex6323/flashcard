//! A simple key-value store for storing flashcard progress.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Represents a stage that a flashcard can be in.
#[derive(Debug)]
pub struct Stage {
    pub index: u64,
    pub timestamp_ms: u64,
}

/// Loads the DB into a hashmap of flashcard hashes, and their respective stages and
/// timestamps when they entered a certain stage.
pub fn load(fname: &str) -> HashMap<u64, Stage> {
    let path = if cfg!(debug_assertions) {
        Path::new("./sample_db.txt")
    } else {
        Path::new(fname)
    };
    let file = if path.exists() {
        File::open(&path).expect("error opening progress database")
    } else {
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(crate::common::fs::get_app_persistence_path())
            .expect("error creating db directory");

        File::create(&path).expect("error creating progress database")
    };
    let buffered = BufReader::new(file);

    let mut result = HashMap::new();

    buffered.lines().filter_map(|r| r.ok()).for_each(|line| {
        let parts = line.split(';').collect::<Vec<&str>>();

        let hash = parts[0].parse::<u64>().expect("error parsing hash");
        let index = parts[1].parse::<u64>().expect("error parsing stage");
        let timestamp_ms = parts[2].parse::<u64>().expect("error parsing unix timestamp");

        //let hash = Hash(hash);
        let stage = Stage { index, timestamp_ms };

        result.insert(hash, stage);
    });

    result
}

/// Saves the progress DB as a file.
pub fn save(db: &HashMap<u64, Stage>, fname: &str) -> std::io::Result<()> {
    let path = if cfg!(debug_assertions) {
        Path::new("./sample_db.txt")
    } else {
        Path::new(fname)
    };
    let file = File::create(&path).expect("error creating db");
    let mut buffered = BufWriter::new(file);

    for (hash, stage) in db.iter() {
        writeln!(buffered, "{};{};{}", hash, stage.index, stage.timestamp_ms)?;
    }

    Ok(())
}

/// Cleans the database by removing all entries that are older then a particular unix
/// time.
///
/// This function is useful to remove dead flashcards (are created when removed from a
/// cardbox or after hash changing modifactions happend)
pub fn clean(fname: &str, older_than: u64) {
    let mut db = load(fname);
    let mut new_db = HashMap::new();

    for (hash, stage) in db.drain() {
        if stage.timestamp_ms >= older_than {
            new_db.insert(hash, stage);
        }
    }

    save(&new_db, fname).expect("error saving progress database");
}

#[cfg(test)]
mod tests {
    use super::super::common::time;
    use super::*;

    // TODO: use different file
    //#[test]
    fn load_and_save_db_with_updated_stage() {
        let mut db = load("./sample_db.txt");
        assert_eq!(20, db.len());

        db.insert(
            9228782626062525010,
            Stage { index: 3, timestamp_ms: time::get_unix_time_millis() },
        );
        assert_eq!(20, db.len());
        save(&db, "./sample_db2.txt");
    }
}
