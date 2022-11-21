use crate::traits::Parse;

pub struct Glob {
    pub pattern: String,
}

impl Glob {
    pub fn new(pattern: String) -> Self {
        Glob { pattern }
    }

    pub fn get(&self) -> Vec<String> {
        return vec!['1'.to_string()];
    }
}

impl Parse for Glob {
    fn init_rule(&self) {}
}
