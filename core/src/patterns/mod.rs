use std::path::PathBuf;

use self::glob::Glob;
use crate::utils::path::normalize_path;
pub mod glob;

pub fn glob_sync(pattern: String) -> Vec<PathBuf> {
    let pattern = normalize_path(&pattern).expect("cannot normalize pattern");
    let glob = Glob::new(pattern);
    return glob.get();
}
