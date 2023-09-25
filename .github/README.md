# mapro

A tiny macro library for creating `std::collections`.

## Overview

`mapro` provides a set of convenient macros for creating various standard library collections, such as `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `VecDeque`, and `BinaryHeap`.

The name `mapro` is a portmanteau of "macro" and "map", but the library handles more than just map types.

## Usage

To use `mapro`, add it to your `Cargo.toml` dependencies:

```toml
[dependencies]
mapro = "0.1.0"
```

or run:
```bash
cargo add mapro
```

Then import the macros you want to use in your Rust files:

```rust
use mapro::{map, bt_map, set, bt_set, vec_d, heap};
```

### Examples

Creating a `HashMap`:

```rust
let m = map!{
    "one" => 1,
    "two" => 2
};

assert_eq!(m["one"], 1);
assert_eq!(m["two"], 2);
```

Creating a `HashSet`:

```rust
let s = set!{1, 2, 3};

assert!(s.contains(&1));
assert!(s.contains(&2));
assert!(s.contains(&3));
```

## Features

- **Simple API**: Using `mapro` is as straightforward as using any Rust macro.

- **No Extra Dependencies**: `mapro` only depends on the standard library.

- **Broad Collection Support**: Supports various standard library collections out of the box.

## Contributing

Interested in contributing? Submit a pull request!

## License

This project is licensed under the MIT License.

## Authors

- Patrick Unick: [dev_storm@winux.com](dev_storm@winux.com)
