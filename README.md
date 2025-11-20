# Tuple Set

[![Documentation](https://docs.rs/tuple_set/badge.svg)](https://docs.rs/tuple_set)
[![CI](https://github.com/LucaCappelletti94/tuple_set/workflows/Rust%20CI/badge.svg)](https://github.com/LucaCappelletti94/tuple_set/actions)
[![Security Audit](https://github.com/LucaCappelletti94/tuple_set/workflows/Security%20Audit/badge.svg)](https://github.com/LucaCappelletti94/tuple_set/actions)
[![Codecov](https://codecov.io/gh/LucaCappelletti94/tuple_set/branch/main/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/tuple_set)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/tuple_set.svg)](https://crates.io/crates/tuple_set)

Ergonomic utilities for working with Rust tuples by unique types, not position.

Supports tuples up to 64 elements, including those with duplicated types, as long as the target type appears exactly once. I might add some feature-gated support for larger tuples in the future.

It comes completely `no-std` compatible, requiring only the `core` crate.

## Example

```rust
use tuple_set::TupleSet;

let mut tuple = (42i32, "hello", None::<&str>, "world", 3.14f64);

// Replace the i32 by type
assert!(tuple.set(100i32).is_none());
assert_eq!(tuple.0, 100);
// The world is not cruel, let's fix that
assert!(tuple.set(Some("cruel")).is_none());
// Now it is
assert_eq!(tuple.2, Some("cruel"));

// Mutate by type
tuple.map(|x: &mut f64| *x *= 2.0);

// Get a reference by type
let value: &f64 = tuple.get().unwrap();
assert_eq!(*value, 6.28);
```

## Limitations

It would be ideal if all methods in this crate were fully verified at compile time, making it impossible to call them in invalid situations and making the `unchecked` variants unnecessary.
Unfortunately, [Rust trait specialization is not stable and does not appear likely to stabilize soon](https://github.com/rust-lang/rust/issues/31844).

Until then, this runtime-checked approach is a practical alternative that still provides strong safety and good ergonomics. The following table summarizes the behavior based on the occurrences of the target type in the tuple:

| Occurrences of target type | Behavior                               |
| -------------------------- | -------------------------------------- |
| Exactly one                | Operation succeeds                     |
| Zero or multiple matches   | `Err` or `None`                        |

## License

MIT License
