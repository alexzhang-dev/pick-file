pub mod task;

use crate::structs::Pattern;

pub struct Task {
    base: String,
    dynamic: bool,
    patterns: Vec<Pattern>,
}
