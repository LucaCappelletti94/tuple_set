# Tuple Set

[![Documentation](https://docs.rs/tuple_set/badge.svg)](https://docs.rs/tuple_set)
[![CI](https://github.com/LucaCappelletti94/tuple_set/workflows/Rust%20CI/badge.svg)](https://github.com/LucaCappelletti94/tuple_set/actions)
[![Security Audit](https://github.com/LucaCappelletti94/tuple_set/workflows/Security%20Audit/badge.svg)](https://github.com/LucaCappelletti94/tuple_set/actions)
[![Codecov](https://codecov.io/gh/LucaCappelletti94/tuple_set/branch/main/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/tuple_set)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/tuple_set.svg)](https://crates.io/crates/tuple_set)

Ergonomic utilities for working with Rust tuples by **unique type**, not by position.

When implementing generic traits, you often care about a type inside a tuple rather than its position. The location may differ across tuple types and may not even be knowable. Tuple Set allows you to operate on tuple values by type when that type appears exactly once.

* Fully `no_std` compatible
* Zero dependencies
* Works with tuples up to 64 elements (feature-gating can extend this)
* Supports duplicated types in the same tuple as long as the target type is unique

## Example

```rust
use tuple_set::TupleSet;

let mut tuple = (42i32, "hello", None::<&str>, "world", 3.14f64);

// Replace the i32 by type
assert!(tuple.set(100i32).is_none());
assert_eq!(tuple.0, 100);

// Make the world cruel
assert!(tuple.set(Some("cruel")).is_none());
assert_eq!(tuple.2, Some("cruel"));

// Mutate by type
tuple.map(|x: &mut f64| *x *= 2.0);

// Get a reference by type
let value: &f64 = tuple.get().unwrap();
assert_eq!(*value, 6.28);
```

## Why this crate exists

Rust tuples are lightweight heterogeneous containers that are very useful in trait-based designs. Modifying an item by position can be cumbersome:

* The position may differ across tuple types
* Trait implementations cannot rely on tuple layout details
* Exposing indexes couples abstractions to layout

Tuple Set lets you express intent in terms of type, leaving layout free to vary. This preserves abstraction and enables ergonomic generic designs.

## Behavior and safety

Tuple Set enforces that a target type must appear exactly once in the tuple. Operations behave as follows:

| Occurrences of target type | Behavior                |
| -------------------------- | ----------------------- |
| Exactly one                | Operation succeeds      |
| Zero or multiple matches   | Returns `Err` or `None` |

Unchecked variants of most methods are provided for cases where correctness is guaranteed by the caller. These exist because fully verifying correctness at compile time would require trait specialization.

Rust trait specialization is not stable and does not appear likely to stabilize soon:
[https://github.com/rust-lang/rust/issues/31844](https://github.com/rust-lang/rust/issues/31844)

Until specialization is stabilized, Tuple Set offers a practical tradeoff: ergonomic, safe, and still performant.

## License

MIT License
