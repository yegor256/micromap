[![cargo](https://github.com/yegor256/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![codecov](https://codecov.io/gh/yegor256/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/micromap)](https://hitsofcode.com/view/github/yegor256/micromap)
![Lines of code](https://img.shields.io/tokei/lines/github/yegor256/micromap)
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
micromap = "0.0.9"
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
| `hashbrown::HashMap` | 11.12 | 2.44 | 1.63 | 0.98 | 0.39 | 0.17 | 0.07 |
| `indexmap::IndexMap` | 13.00 | 5.83 | 3.20 | 1.97 | 0.76 | 0.33 | 0.16 |
| `linear_map::LinearMap` | 1.98 | 0.62 | 0.61 | 0.61 | 0.44 | 0.41 | 0.43 |
| `linked_hash_map::LinkedHashMap` | 36.84 | 11.39 | 4.83 | 3.01 | 1.52 | 0.70 | 0.33 |
| `litemap::LiteMap` | 1.97 | 1.10 | 0.84 | 0.71 | 0.41 | 0.26 | 0.19 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 11.46 | 2.34 | 1.69 | 0.62 | 0.28 | 0.12 | 0.06 |
| `rustc_hash::FxHashMap` | 11.85 | 2.27 | 1.70 | 0.71 | 0.27 | 0.13 | 0.06 |
| `std::collections::BTreeMap` | 30.43 | 4.81 | 2.90 | 2.42 | 1.06 | 0.48 | 0.26 |
| `std::collections::HashMap` | 21.49 | 9.85 | 4.48 | 2.14 | 0.87 | 0.66 | 0.26 |
| `tinymap::array_map::ArrayMap` | 1.50 | 1.87 | 2.19 | 2.72 | 2.63 | 2.55 | 2.47 |

The experiment [was performed](https://github.com/yegor256/micromap/actions/workflows/benchmark.yml) on 25-04-2023.
There were 1000000 repetition cycles.
The entire benchmark took 159s.

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
