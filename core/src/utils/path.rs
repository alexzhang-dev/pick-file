use regex::{Error, Regex};

pub fn remove_duplicate_slashes(path: &String) -> Result<String, Error> {
    let re = Regex::new(r"/{2,}");
    match re {
        Ok(regex) => {
            let res = regex
                .replace_all(path.clone().as_mut_str(), "/")
                .to_string();
            Ok(res)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test() {
        let str = "C://test/////echo".to_string();
        let str = remove_duplicate_slashes(&str).unwrap();
        assert_eq!(str, "C:/test/echo".to_string());
    }
}
