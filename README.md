[![cargo](https://github.com/yegor256/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![codecov](https://codecov.io/gh/yegor256/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/micromap)](https://hitsofcode.com/view/github/yegor256/micromap)
![Lines of code](https://img.shields.io/tokei/lines/github/yegor256/micromap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/micromap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/micromap)](https://docs.rs/micromap/latest/micromap/)

At least **5x faster** alternative of [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), 
for very small maps. It is also faster than
[FxHashMap](https://github.com/rust-lang/rustc-hash),
[ArrayMap](https://github.com/robjtede/tinymap). The smaller the map, the higher the
performance. When the map is larger than 50, it is better to use standard 
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html).

**WELCOME**: 
Not all functions that a user expects to have in a map are implemented. 
I will appreciate if you contribute by implementing these missing functions.

Here is how you use it:

```rust
use micromap::Map;
let mut m : Map<u64, &str, 10> = Map::new();
m.insert(1, "foo");
m.insert(2, "bar");
assert_eq!(2, m.len());
```

Pay attention, here the map is created with an extra generic argument `10`. This is 
the total size of the map, which is allocated on stack when `::new()` is called. 
Unlike `HashMap`, the `Map` doesn't use heap at all.

Read [the API documentation](https://docs.rs/micromap/latest/micromap/). Important to notice
that signatures of some functions are different from `HashMap`, for example `remove()` and
`insert()` expect arguments to be passes by-value instead of by-reference.

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
