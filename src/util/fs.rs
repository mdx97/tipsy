//! Utility functions that pertain to the file system

extern crate dirs;

use std::path::PathBuf;
use std::path::Path;

use crate::consts::TIPSY_DIRECTORY;

/// Returns the path to the tipsy directory (e.g. ~/.tipsy).
/// This function is used because of the inability to const eval `dirs::home_dir`.
pub fn tipsy_path() -> Box<Path> {
    let mut path = PathBuf::new();
    path.push(dirs::home_dir().expect("Failed to get home directory"));
    path.push(TIPSY_DIRECTORY);
    path.into_boxed_path()
}


