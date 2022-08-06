# iter_accumulate
An iterator adaptor for Rust that accumulates the elements from the base iterator
using the provided closure.

This is similar to `fold()`, but instead of returning the final accumulated result, it returns an
iterator that yields the current accumulated value for each iteration. In other words, the last
element yielded by `accumulate()` is what would have been returned by `fold()` if it was used
instead.

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