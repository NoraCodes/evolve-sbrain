//! Randomness functionality for this program

use super::random;
use random::Source;
use std::time::*;

// Create a randomness source based on the current time.
pub fn get_randomness() -> random::Default {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    random::default().seed([t.as_secs(), t.subsec_nanos() as u64])
}