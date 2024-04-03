# iter_accumulate
An iterator adaptor for Rust that accumulates the elements from the base iterator
using the provided closure.

## Example

```rust
use iter_accumulate::IterAccumulate;

let input = [1, 2, 3, 4, 5];
let mut iter = input.iter().accumulate(1, |acc, i| acc * i);

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), Some(24));
assert_eq!(iter.next(), Some(120));
assert_eq!(iter.next(), None);
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
