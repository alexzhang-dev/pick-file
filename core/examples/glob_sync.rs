use pick_file::patterns::glob_sync;

fn main() {
    let result = glob_sync("../".to_string());
    println!("{:?}", result);
}
