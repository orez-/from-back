# from-back

A Rust library for indexing and slicing from the back of a sequence.

```rust
use from_back::idx;

let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
let slice = &list[idx!(2..^3)];
assert_eq!(slice, &[2, 3, 4, 5, 6]);
```
