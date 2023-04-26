// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! This is a simpler and faster alternative implementation of the standard `HashMap`.
//! It doesn't use heap and doesn't use hashing at all. It simply keeps all key-value
//! pairs in an array and when it's necessary to retrieve by key, it scrolls through
//! the entire array. This implementation works much faster for small maps of
//! less than 50 keys, but definitely is not suitable for larger maps.
//!
//! For example, here is a map with a few keys can be created:
//!
//! ```
//! use micromap::Map;
//! let mut m : Map<u64, &str, 10> = Map::new();
//! m.insert(1, "Hello, world!");
//! m.insert(2, "Good bye!");
//! assert_eq!(2, m.len());
//! ```
//!
//! Creating a [`Map`] requires knowing the maximum size of it upfront. This is
//! what the third type argument `10` is for, in the example above. The array
//! will have exactly ten elements. An attempt to add an 11th element will lead
//! to a panic.

#![doc(html_root_url = "https://docs.rs/micromap/0.0.11")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::multiple_crate_versions)]

mod clone;
mod ctors;
mod debug;
mod eq;
mod from;
mod index;
mod iterators;
mod map;
mod pair;
#[cfg(feature = "serde")]
mod serialization;

use std::mem::MaybeUninit;

/// A pair in the Map.
#[derive(Clone, Default, Eq, PartialEq)]
enum Pair<K, V> {
    Present((K, V)),
    #[default]
    Absent,
}

/// A faster alternative of [`std::collections::HashMap`].
///
/// It is faster because it doesn't use a hash function at all. It simply keeps
/// all pairs in an array and when it's necessary to find a value, it goes through
/// all pairs comparing the needle with each pair available. Also it is faster
/// because it doesn't use heap. When a [`Map`] is being created, it allocates the necessary
/// space on stack. That's why the maximum size of the map must be provided in
/// compile time.
pub struct Map<K: Clone + PartialEq, V: Clone, const N: usize> {
    next: usize,
    pairs: [MaybeUninit<Pair<K, V>>; N],
}

/// Iterator over the [`Map`].
pub struct Iter<'a, K, V, const N: usize> {
    next: usize,
    pos: usize,
    pairs: &'a [MaybeUninit<Pair<K, V>>; N],
}

/// Into-iterator over the [`Map`].
pub struct IntoIter<'a, K, V, const N: usize> {
    next: usize,
    pos: usize,
    pairs: &'a [MaybeUninit<Pair<K, V>>; N],
}

#[test]
fn map_can_be_cloned() {
    let mut m: Map<u8, u8, 8> = Map::new();
    m.insert(0, 42);
    assert_eq!(42, *m.clone().get(&0).unwrap());
}
