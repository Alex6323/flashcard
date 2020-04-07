//! Filesystem related common utility functions.

use crate::constants::{APP_NAME, DB_NAME};

/// Return the application's directory for persistent data storage.
pub fn get_app_persistence_path() -> String
{
    let path = dirs::home_dir().expect("error retreiving home directory");
    let path = path.join(APP_NAME);
    format!("{}", path.display())
}

/// Return the path of the 'progress.db' file for storing progress.
pub fn get_progress_db_path() -> String
{
    let path = dirs::home_dir().expect("error retreiving home directory");
    let path = path.join(APP_NAME).join(DB_NAME);
    format!("{}", path.display())
}
