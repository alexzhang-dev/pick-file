use std::path::Path;

use pick_file::helpers::fs::{fs_walker, WalkerResult};

fn main() {
    let mut walker_result = WalkerResult { paths: Vec::new() };
    let re = fs_walker(Path::new("./core/src"), &mut walker_result);
    match re {
        Ok(_) => {
            println!("{:?}", walker_result.paths)
        }
        Err(_) => (),
    }
}
