use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;
use std::fs;
use std::path::PathBuf;
use tokio::spawn;

/// Represents the state of the rules, including whether any rules are broken.
#[derive(Serialize)]
pub struct State {
    pub rules_broken: bool,
}

impl State {
    /// Creates a new State instance.
    pub fn new(rules_broken: bool) -> Self {
        State { rules_broken }
    }
}

/// Checks the rules and performs actions based on the state.
///
/// If rules are broken, schedules autodelete of all files in `target_directory` after 6 hours.
///
/// # Arguments
///
/// * `state` - A reference to the State structure containing the rules state.
/// * `target_directory` - The directory path where files should be deleted.
///
/// # Important:
/// The `target_directory` itself must never be removed or deleted to prevent
/// accidental encryption or deletion of wrong files or directories.
/// Only files inside this directory will be deleted.
pub fn check_rules(state: &State, target_directory: PathBuf) {
    if state.rules_broken {
        // Spawn a background asynchronous task to wait 6 hours then delete files
        spawn(async move {
            // Wait for 6 hours (21600 seconds)
            sleep(Duration::from_secs(6 * 60 * 60)).await;

            // Attempt to read the directory entries
            if let Ok(entries) = fs::read_dir(&target_directory) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    // Only delete files, never delete directories or the directory itself
                    if path.is_file() {
                        if let Err(e) = fs::remove_file(&path) {
                            eprintln!("Failed to delete file {:?}: {}", path, e);
                        }
                    }
                }
            } else {
                eprintln!("Failed to read target directory {:?}", target_directory);
            }
        });
    }
}
