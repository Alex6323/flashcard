//! Time related common utiliy functions.

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the unix time.
pub fn get_unix_time_millis() -> u64
{
    let unix_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("error determining system time");

    unix_time.as_secs() * 1000 + u64::from(unix_time.subsec_millis())
}
