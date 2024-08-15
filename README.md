# Prop-test

Utility for the [proptest](https://crates.io/crates/proptest) crate,
which provides a macro that is formattable with rustfmt.

Unlike the original `proptest!()` macro, this macro isn't expected to "wrap" test functions
but is instead intended to be placed as expressions within them.

# Example

```rust
fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    xs.iter().rev().cloned().collect()
}

use prop_test::prelude::*;

// This expression would be inside of a `#[test]` function
prop_test!(&prop::collection::vec(any::<i32>(), 0..100), |xs| {
    prop_assert_eq!(&xs, &reverse(&reverse(&xs)));
    Ok(())
});
```
