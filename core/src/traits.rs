use std::path::PathBuf;

pub trait Parse {
    fn parse_rule(&self) -> PathBuf;
}
