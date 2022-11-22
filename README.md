# Pick-File

Collect essential utilities for file operating.

## Glob Pattern(not release yet)

### Usage

```rs
fn main() {
    let glob = Glob::new("src/**/*.rs");
    glob.pick(); // returns Vec<PathBuf>
}
```

## is-glob

[Doc](./is-glob/README.md)
