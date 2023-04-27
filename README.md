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
micromap = "0.0.12"
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
| `hashbrown::HashMap` | 13.45 | 7.07 | 3.24 | 1.85 | 0.80 | 0.40 | 0.20 |
| `indexmap::IndexMap` | 11.76 | 12.26 | 5.66 | 3.67 | 1.62 | 0.81 | 0.41 |
| `linear_map::LinearMap` | 1.73 | 1.37 | 0.81 | 0.99 | 0.79 | 0.91 | 0.80 |
| `linked_hash_map::LinkedHashMap` | 22.53 | 16.47 | 8.09 | 4.82 | 2.29 | 1.18 | 0.58 |
| `litemap::LiteMap` | 3.75 | 2.65 | 1.42 | 1.21 | 0.68 | 0.41 | 0.28 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 10.35 | 7.14 | 3.71 | 1.55 | 0.74 | 0.38 | 0.19 |
| `rustc_hash::FxHashMap` | 10.21 | 6.68 | 3.16 | 2.05 | 0.77 | 0.39 | 0.20 |
| `std::collections::BTreeMap` | 17.03 | 8.62 | 4.67 | 3.65 | 1.84 | 0.84 | 0.52 |
| `std::collections::HashMap` | 26.54 | 12.02 | 5.72 | 3.51 | 1.69 | 0.88 | 0.44 |
| `tinymap::array_map::ArrayMap` | 0.94 | 4.32 | 3.14 | 3.47 | 3.38 | 3.28 | 3.14 |

The experiment [was performed](https://github.com/yegor256/micromap/actions/workflows/benchmark.yml) on 27-04-2023.
There were 1000000 repetition cycles.
The entire benchmark took 343s.

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
