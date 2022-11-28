use is_glob::is_glob;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ParseResult {}

lazy_static! {
    static ref CACHE: HashMap<String, ParseResult> = {
        let m = HashMap::new();
        m
    };
}

pub fn parse(pattern: &String) -> Option<ParseResult> {
    if !is_glob(pattern) {
        return None;
    }

    let cache = CACHE.get(pattern);

    if let Some(cache) = cache {
        return Some((*cache).clone());
    }

    return Some(ParseResult {});
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
