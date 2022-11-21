# Pick-File

Traverse local files by some specific rules.

## Glob

### Usage

```rs
fn main() {
    let glob = Glob::new("src/**/*.rs");
    glob.pick(); // returns Vec<PathBuf>
}
```
