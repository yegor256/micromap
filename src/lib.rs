// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! This is a simpler and faster alternative implementation of the standard `HashMap`.
//! It doesn't use heap and doesn't use hashing at all. It simply keeps all key-value
//! pairs in an array and when it's necessary to retrieve by key, it scrolls through
//! the entire array. This implementation works much faster for small maps of
//! less than 50 keys, but definitely is not suitable for larger maps.
//!
//! Check [this page](https://github.com/yegor256/micromap#benchmark)
//! for the recent benchmarking results.
//!
//! For example, here is how a map with a few keys can be created:
//! ```
//! use micromap::Map;
//! let mut m : Map<u64, &str, 10> = Map::new();
//! m.insert(1, "Hello, world!");
//! m.insert(2, "Good bye!");
//! assert_eq!(m.len(), 2);
//! assert_eq!(m.capacity(), 10);
//! ```
//!
//! Creating a [`Map`] requires knowing the maximum size of it, upfront. This is
//! what the third type argument `10` is for, in the example above. The array
//! will have exactly ten elements. An attempt to [`insert`][Map::insert] an 11th
//! element will lead to a panic. (Or use [`checked_insert`][Map::checked_insert]
//! instead to avoid panics by returning an [`Option`].)

#![cfg_attr(all(not(feature = "std"), not(doc), not(test)), no_std)]
#![doc(html_root_url = "https://docs.rs/micromap/0.0.0")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(rust_2018_idioms)]
// About the docs
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![warn(rustdoc::missing_crate_level_docs)]
// Our Goal, uncomment these!
// #![warn(missing_docs)]
// #![doc(test(attr(deny(unused))))]
#![doc(test(attr(warn(unused))))]

pub mod map;
pub mod set;

// re-export Set
pub use map::Map;
pub use set::Set;
