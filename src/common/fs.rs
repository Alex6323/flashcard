//! Filesystem related common utility functions.

use crate::constants::{APP_NAME, DB_NAME};

pub fn get_progress_db_path() -> String {
    let path = dirs::home_dir().expect("error retreiving home directory");
    let path = path.join(APP_NAME).join(DB_NAME);
    format!("{}", path.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn progress_db_path() {
        println!("{}", get_progress_db_path());
    }
}
