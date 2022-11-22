use std::{
    fs::read_dir,
    io,
    path::{Path, PathBuf},
};

pub struct WalkerResult {
    pub paths: Vec<PathBuf>,
}

pub fn fs_walker(base_dir: &Path, walker_result: &mut WalkerResult) -> io::Result<()> {
    if base_dir.is_dir() {
        for entry in read_dir(base_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.starts_with("./.git") || path.starts_with("./target") {
                continue;
            }
            if path.is_dir() {
                fs_walker(&path, walker_result)?;
            } else {
                walker_result.paths.push(path);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn fs_walker_test() {
        let mut walker_result = WalkerResult { paths: Vec::new() };
        let re = fs_walker(Path::new("../"), &mut walker_result);
        match re {
            Ok(_) => {
                println!("{:?}", walker_result.paths)
            }
            Err(_) => (),
        }
    }
}
