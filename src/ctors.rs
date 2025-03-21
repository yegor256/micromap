// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use core::mem::MaybeUninit;

impl<K: PartialEq, V, const N: usize> Default for Map<K, V, N> {
    /// Make a default empty [`Map`].
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialEq, V, const N: usize> Map<K, V, N> {
    /// Make it.
    ///
    /// The size of the map is defined by the generic argument. For example,
    /// this is how you make a map of four key-values pairs:
    #[inline]
    #[must_use]
    #[allow(clippy::uninit_assumed_init)]
    pub const fn new() -> Self {
        unsafe {
            Self {
                len: 0,
                pairs: MaybeUninit::<[MaybeUninit<(K, V)>; N]>::uninit().assume_init(),
            }
        }
    }
}

impl<K: PartialEq, V, const N: usize> Drop for Map<K, V, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            self.item_drop(i);
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
