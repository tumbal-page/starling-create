use std::path::PathBuf;
use std::{fs, env};

pub(super) static LOG_TEST: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Returns a fresh directory for a test fixture named `name`.
pub(super) fn dir(name: &str) -> PathBuf {
    let dir = env::temp_dir().join("starling-native-mods-tests").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}
