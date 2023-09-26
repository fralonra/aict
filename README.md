# aict

[![Latest version](https://img.shields.io/crates/v/aict.svg)](https://crates.io/crates/aict)
[![Documentation](https://docs.rs/aict/badge.svg)](https://docs.rs/aict)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

`aict` is designed for generating auto-incrementing unique IDs of a specific type.

It provides built-in support for integer types (`i8`, `i16`, `i32`, `i64`, `isize`, `u8`, `u16`, `u32`, `u64`, `usize`). However, you can add support for your own types by implementing the [`Aictable`](https://docs.rs/aict/latest/aict/struct.Aictable.html) trait.

## Examples

```rust
use aict::Factory;

// Creates a new Factory for u32 IDs.
let mut factory = Factory::<u32>::builder()
    // Sets the initial value for the IDs.
    // For built-in types, the default value is the minimum value.
    // .initial_value(1)

    // Whether to loop back to the initial value after reaching the maximum value.
    // Default is false.
    // .looping(true)

    // Whether to rewind to the position of the latest removed ID when generating the next ID.
    // Default is false.
    // .rewind(true)

    .build();

// Generates some IDs.
assert_eq!(factory.next().unwrap(), 0);
assert_eq!(factory.next().unwrap(), 1);

// Removes an ID so that it can be reused in the future.
factory.remove(0);

// Manually marks an ID as used.
assert!(factory.take_up(2).is_ok());
// Since 2 was taken up, the next available ID is 3.
// However, if rewind is set to true, the next ID is 0.
assert_eq!(factory.next().unwrap(), 3);
```