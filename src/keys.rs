// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{IntoKeys, Keys, Map};
use core::iter::FusedIterator;

impl<K: PartialEq, V, const N: usize> Map<K, V, N> {
    /// An iterator visiting all keys in arbitrary order.
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { iter: self.iter() }
    }

    /// Consuming iterator visiting all keys in arbitrary order.
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V, N> {
        IntoKeys {
            iter: self.into_iter(),
        }
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| p.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K: PartialEq, V, const N: usize> Iterator for IntoKeys<K, V, N> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<K> {
        self.iter.next().map(|p| p.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K, V> ExactSizeIterator for Keys<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: PartialEq, V, const N: usize> ExactSizeIterator for IntoKeys<K, V, N> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> FusedIterator for Keys<'_, K, V> {}

impl<K: PartialEq, V, const N: usize> FusedIterator for IntoKeys<K, V, N> {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iterate_keys() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("foo".to_string(), 0);
        m.insert("bar".to_string(), 0);
        let keys = m.keys();
        assert_eq!(keys.len(), 2);
        assert_eq!(keys.collect::<Vec<_>>(), [&"foo", &"bar"]);
    }

    #[test]
    fn iterate_into_keys() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("foo".to_string(), 0);
        m.insert("bar".to_string(), 0);
        let keys = m.into_keys();
        assert_eq!(keys.len(), 2);
        assert_eq!(
            keys.collect::<Vec<_>>(),
            ["bar".to_string(), "foo".to_string()]
        );
    }
}
