pub mod glob;

#[cfg(test)]
pub mod test {
    use super::glob::Glob;

    #[test]
    pub fn glob_test() {
        let glob = Glob::new("**/**/mod".to_string());
        assert_eq!(glob.get(), vec!['1'.to_string()]);
    }
}
