[![cargo](https://github.com/yegor256/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![codecov](https://codecov.io/gh/yegor256/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/micromap)](https://hitsofcode.com/view/github/yegor256/micromap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/micromap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/micromap)](https://docs.rs/micromap/latest/micromap/)

A much faster alternative of [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), 
for very small maps. 
It is also faster than
[FxHashMap](https://github.com/rust-lang/rustc-hash),
[hashbrown](https://github.com/rust-lang/hashbrown),
[ArrayMap](https://github.com/robjtede/tinymap),
[IndexMap](https://crates.io/crates/indexmap),
and _all_ others.
The smaller the map, the higher the performance. 
It was observed that when a map contains more than 20 keys, it may be better to use the standard 
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), since
the performance of `micromap::Map` _may_ start to degrade. 
See the [benchmarking results](#benchmark) below.

**WELCOME**: 
Not all functions that you might expect to have in a map are implemented. 
I will appreciate if you contribute by implementing these 
[missing functions](https://github.com/yegor256/micromap/issues).

First, add this to `Cargo.toml`:

```toml
[dependencies]
micromap = "0.0.14"
```

Then, use it like a standard hash map... well, almost:

```rust
use micromap::Map;
let mut m : Map<u64, &str, 10> = Map::new(); // allocation on stack
m.insert(1, "foo");
m.insert(2, "bar");
assert_eq!(2, m.len());
```

Pay attention, here the map is created with an extra generic argument `10`. This is 
the total size of the map, which is allocated on stack when `::new()` is called. 
Unlike `HashMap`, the `Map` doesn't use heap at all. If more than ten keys will be
added to the map, it will panic.

Read [the API documentation](https://docs.rs/micromap/latest/micromap/). The struct
[`micromap::Map`](https://docs.rs/micromap/latest/micromap/struct.Map.html) is designed as closely similar to 
[`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `micromap::Map` with
a few other Rust maps, changing the total capacity of the map (horizontal axis).
We applied the same interactions 
([`benchmark.rs`](https://github.com/yegor256/micromap/blob/master/tests/benchmark.rs)) 
to them and measured how fast they performed. In the following table, 
the numbers over 1.0 indicate performance gain, 
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 2 | 4 | 8 | 16 | 32 | 64 | 128 |
| --- | --: | --: | --: | --: | --: | --: | --: |
| `hashbrown::HashMap` | 17.12 | 9.15 | 4.80 | 2.27 | 0.93 | 0.39 | 0.20 |
| `heapless::LinearMap` | 0.80 | 1.24 | 0.82 | 0.85 | 0.64 | 0.65 | 0.63 |
| `indexmap::IndexMap` | 10.46 | 9.92 | 5.55 | 2.91 | 1.36 | 0.60 | 0.32 |
| `linear_map::LinearMap` | 1.80 | 1.27 | 0.83 | 0.68 | 0.88 | 0.71 | 0.74 |
| `linked_hash_map::LinkedHashMap` | 22.14 | 16.74 | 8.80 | 4.62 | 2.12 | 0.96 | 0.51 |
| `litemap::LiteMap` | 3.01 | 2.32 | 1.59 | 1.13 | 0.72 | 0.41 | 0.28 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 16.53 | 9.45 | 5.47 | 2.06 | 0.93 | 0.43 | 0.22 |
| `rustc_hash::FxHashMap` | 16.46 | 9.23 | 5.36 | 2.44 | 0.78 | 0.36 | 0.20 |
| `std::collections::BTreeMap` | 16.97 | 7.80 | 6.02 | 3.94 | 2.17 | 0.78 | 0.48 |
| `std::collections::HashMap` | 17.13 | 11.76 | 6.56 | 3.31 | 1.57 | 0.72 | 0.37 |
| `tinymap::array_map::ArrayMap` | 1.60 | 3.67 | 3.31 | 2.98 | 3.12 | 2.98 | 3.04 |

The experiment [was performed](https://github.com/yegor256/micromap/actions/workflows/benchmark.yml) on 20-12-2023.
There were 1000000 repetition cycles.
The entire benchmark took 201s.

<!-- benchmark -->

As you see, the highest performance gain was achieved for the maps that were smaller than ten keys.
For the maps of just a few keys, the gain was enormous.

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
$ cargo test -vv
```

If everything goes well, fork repository, make changes, send us a [pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also, 
run `cargo fmt` and `cargo clippy`.

Also, before you start making changes, run benchmarks:

```bash
$ rustup run nightly cargo bench
```

Then, after the changes you make, run it again. Compare the results. If your changes
degrade performance, think twice before submitting a pull request.
