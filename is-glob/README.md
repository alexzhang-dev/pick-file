# is-glob

check input string if it's a glob pattern.

Returns true/false.

```rs
use is_glob::is_glob;

assert_eq!(is_glob("*.js"), true); // true
assert_eq!(is_glob("~/abc"), false); // true
```
