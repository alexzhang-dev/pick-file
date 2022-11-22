use crate::{
    helpers::fs::{fs_walker, WalkerResult},
    traits::Parse,
};
use std::path::{Path, PathBuf};

pub struct Glob {
    pub pattern: String,
}

impl Glob {
    pub fn new(pattern: String) -> Self {
        Glob { pattern }
    }

    pub fn get(&self) -> Vec<PathBuf> {
        let base_dir = self.parse_rule();
        let mut walker_result = WalkerResult { paths: Vec::new() };
        fs_walker(base_dir.as_path(), &mut walker_result).expect("Failed to walk the file system");
        return walker_result.paths;
    }
}

impl Parse for Glob {
    fn parse_rule(&self) -> PathBuf {
        return Path::new("./").to_path_buf();
    }
}
