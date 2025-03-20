# The Fastest Linear Map in Rust

[![cargo](https://github.com/yegor256/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![codecov](https://codecov.io/gh/yegor256/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/micromap)](https://hitsofcode.com/view/github/yegor256/micromap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/micromap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/micromap)](https://docs.rs/micromap/latest/micromap/)

A much faster alternative of
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
for very small maps.
It is also faster than
[FxHashMap](https://github.com/rust-lang/rustc-hash),
[hashbrown](https://github.com/rust-lang/hashbrown),
[ArrayMap](https://github.com/robjtede/tinymap),
[IndexMap](https://crates.io/crates/indexmap),
and _all_ others.
The smaller the map, the higher the performance.
It was observed that when a map contains more than 20 keys,
it may be better to use the standard
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
since the performance of `micromap::Map` _may_ start to degrade.
See the [benchmarking results](#benchmark) below.

**WELCOME**:
Not all functions that you might expect to have in a map are implemented.
I will appreciate if you contribute by implementing these
[missing functions](https://github.com/yegor256/micromap/issues).

First, add this to `Cargo.toml`:

```toml
[dependencies]
micromap = "0.0.15"
```

Then, use it like a standard hash map... well, almost:

```rust
use micromap::Map;
let mut m : Map<u64, &str, 10> = Map::new(); // allocation on stack
m.insert(1, "foo");
m.insert(2, "bar");
assert_eq!(2, m.len());
```

Pay attention, here the map is created with an extra generic argument `10`.
This is the total size of the map, which is allocated on stack when `::new()`
is called. Unlike `HashMap`, the `Map` doesn't use heap at all. If more than
ten keys will be added to the map, it will panic.

Read [the API documentation](https://docs.rs/micromap/latest/micromap/).
The struct
[`micromap::Map`](https://docs.rs/micromap/latest/micromap/struct.Map.html)
is designed as closely similar to
[`std::collections::HashMap`][std] as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `micromap::Map` with
a few other Rust maps, changing the total capacity of the map (horizontal axis).
We applied the same interactions
([`benchmark.rs`][rs])
to them and measured how fast they performed. In the following table,
the numbers over 1.0 indicate performance gain,
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 2 | 4 | 8 | 16 | 32 | 64 | 128 |
| --- | --: | --: | --: | --: | --: | --: | --: |
| `flurry::HashMap` | 282.69 | 92.25 | 42.85 | 14.37 | 9.70 | 4.91 | 2.69 |
| `hashbrown::HashMap` | 20.55 | 12.30 | 6.92 | 2.06 | 1.28 | 0.68 | 0.32 |
| `heapless::LinearMap` | 1.08 | 1.59 | 1.24 | 0.76 | 1.03 | 1.25 | 1.00 |
| `indexmap::IndexMap` | 13.31 | 13.04 | 7.62 | 2.75 | 1.73 | 0.93 | 0.49 |
| `linear_map::LinearMap` | 1.62 | 1.67 | 1.08 | 0.61 | 0.75 | 1.03 | 0.87 |
| `linked_hash_map::LinkedHashMap` | 25.68 | 22.75 | 12.71 | 4.29 | 2.79 | 1.46 | 0.77 |
| `litemap::LiteMap` | 1.62 | 2.42 | 5.45 | 2.28 | 1.76 | 0.93 | 0.58 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 20.55 | 12.58 | 7.41 | 1.98 | 1.19 | 0.62 | 0.35 |
| `rustc_hash::FxHashMap` | 20.53 | 12.39 | 7.09 | 2.34 | 1.03 | 0.59 | 0.31 |
| `std::collections::BTreeMap` | 19.61 | 10.54 | 5.48 | 2.57 | 1.97 | 1.07 | 0.78 |
| `std::collections::HashMap` | 20.77 | 15.76 | 9.01 | 2.97 | 2.09 | 1.04 | 0.55 |
| `tinymap::array_map::ArrayMap` | 1.97 | 4.65 | 5.15 | 2.80 | 3.84 | 4.43 | 4.80 |

The experiment [was performed][action] on 19-03-2025.
There were 1000000 repetition cycles.
The entire benchmark took 251s.
Uname: 'Linux'.

<!-- benchmark -->

As you see, the highest performance gain was achieved for the maps that
were smaller than ten keys.
For the maps of just a few keys, the gain was enormous.

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
cargo test -vv
```

If everything goes well, fork repository, make changes, send us a
[pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also,
run `cargo fmt` and `cargo clippy`.

Also, before you start making changes, run benchmarks:

```bash
rustup run nightly cargo bench
```

Then, after the changes you make, run it again. Compare the results.
If your changes
degrade performance, think twice before submitting a pull request.

[std]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[rs]: https://github.com/yegor256/micromap/blob/master/tests/benchmark.rs
[action]: https://github.com/yegor256/micromap/actions/workflows/benchmark.yml
