use regex::{Error, Regex};

pub fn modify_by_re(
    original: &String,
    reg: Result<Regex, Error>,
    replace_str: &str,
) -> Result<String, Error> {
    match reg {
        Ok(regex) => {
            let res = regex
                .replace_all(original.clone().as_mut_str(), replace_str)
                .to_string();
            Ok(res)
        }
        Err(err) => Err(err),
    }
}

// from 'c/////x' to 'c/x'
pub fn remove_duplicate_slashes(path: &String) -> Result<String, Error> {
    let re = Regex::new(r"/{2,}");
    modify_by_re(path, re, "/")
}

// from 'x\\xx' to 'x/x'
pub fn normalize_path(path: &String) -> Result<String, Error> {
    let re = Regex::new(r"\\");
    match modify_by_re(path, re, "/") {
        Ok(res) => remove_duplicate_slashes(&res),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn should_remove_duplicate_slashes() {
        let str = "C://test/////echo".to_string();
        let str = remove_duplicate_slashes(&str).unwrap();
        assert_eq!(str, "C:/test/echo".to_string());
    }

    #[test]
    pub fn should_normalize_path() {
        let str = r"x\\\\\x".to_string();
        let str = normalize_path(&str).unwrap();
        assert_eq!(str, "x/x".to_string());
    }
}
