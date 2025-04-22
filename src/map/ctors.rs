// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Map;
use core::mem::MaybeUninit;

impl<K, V, const N: usize> Default for Map<K, V, N> {
    /// Creates a empty [Map] like [`new()`][`Map::new`].
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, const N: usize> Map<K, V, N> {
    /// Creates an empty (len) Linear Map with capacity `N`.
    ///
    /// The linear map is initially created with a place that has a capacity
    /// of `N` key-value pairs (and one usize), so it will immediately occupy
    /// these memory on the stack (no allocation on heap).
    ///
    /// After creation, capacity will not change any more, which is the max
    /// len of the map.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let map: Map<&str, i32, 20> = Map::new();
    /// assert_eq!(map.capacity(), 20);
    /// assert_eq!(map.len(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            len: 0,
            pairs: [const { MaybeUninit::uninit() }; N],
        }
    }

    /// Creates an empty [Map] with fixed capacity.
    ///
    /// The map will be able to hold at most `capacity` elements. And
    /// the argument `capacity` should be always equal to the generic
    /// constant `N`.
    ///
    /// # Panics
    /// If `capacity` is not equal to `N`, the function will panic.
    ///
    /// # Examples
    /// ```should_panic
    /// # #![allow(deprecated)]
    /// use micromap::Map;
    /// let map: Map<&str, i32, 10> = Map::with_capacity(10); // correct
    /// assert_eq!(map.capacity(), 10);
    /// let map: Map<&str, i32, 10> = Map::with_capacity(20); // panic here
    /// assert_eq!(map.capacity(), 10); // unreachable line
    /// ```
    #[deprecated(note = "Please use `new()` instead.")]
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity == N, "capacity must be equal to N");
        Self::new()
    }
}

impl<K, V, const N: usize> Drop for Map<K, V, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe { self.item_drop(i) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_default_map() {
        let m: Map<u8, u8, 8> = Map::default();
        assert_eq!(0, m.len());
    }

    #[test]
    fn makes_new_map() {
        let m: Map<u8, u8, 8> = Map::new();
        assert_eq!(0, m.len());
    }

    #[test]
    fn drops_correctly() {
        let _m: Map<Vec<u8>, u8, 8> = Map::new();
    }

    #[test]
    fn drops_keys() {
        use std::rc::Rc;
        let mut m: Map<Rc<()>, (), 8> = Map::new();
        let k = Rc::new(());
        m.insert(Rc::clone(&k), ());
        drop(m);
        assert_eq!(Rc::strong_count(&k), 1);
    }

    #[test]
    fn drops_values() {
        use std::rc::Rc;
        let mut m: Map<(), Rc<()>, 8> = Map::new();
        let v = Rc::new(());
        m.insert((), Rc::clone(&v));
        drop(m);
        assert_eq!(Rc::strong_count(&v), 1);
    }
}
