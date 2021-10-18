# rust-cache-control
Rust crate to parse the HTTP Cache-Control header.

```rust
use cache_control::{Cachability, CacheControl};
use std::time::Duration;

let cache_control = CacheControl::from_header("Cache-Control: public, max-age=60").unwrap();
assert_eq!(cache_control.cachability, Some(Cachability::Public));
assert_eq!(cache_control.max_age, Some(Duration::from_secs(60)));
```
