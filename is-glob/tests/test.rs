#[cfg(test)]
mod tests {
    use is_glob::is_glob;
    #[test]
    fn should_be_true_if_valid_pattern() {
        let patterns = vec!["*.js", "!*.js", "!foo", "!foo.js", "**/abc.js", "abc/*.js"];
        for pattern in patterns {
            assert!(is_glob(&pattern.to_string()));
        }
    }
    #[test]
    fn should_not_match_escaped_globs() {
        let patterns = vec![
            "\\!\\*.js",
            "\\!foo",
            "\\!foo.js",
            "\\*.js",
            "\\*\\*/abc.js",
            "abc/\\*.js",
        ];
        for pattern in patterns {
            assert!(!is_glob(&pattern.to_string()));
        }
    }
    #[test]
    fn should_be_false_if_glob_invalid() {
        let patterns = vec![
            "",
            "~/abc",
            "~abc",
            "~/(abc)",
            "+~(abc)",
            ".",
            "aa",
            "who",
            "why?",
            "abc!/def/!ghi.js",
            "abc.js",
            "abc/def/!ghi.js",
            "abd/def/ghi.js",
        ];
        for pattern in patterns {
            assert!(!is_glob(&pattern.to_string()));
        }
    }
}
