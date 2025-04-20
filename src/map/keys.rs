// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::iterators::{IntoIter, Iter};
use super::Map;
use core::fmt;
use core::iter::FusedIterator;

impl<K, V, const N: usize> Map<K, V, N> {
    /// An iterator visiting all keys in arbitrary order. The iterator element
    /// type is `&'a K`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut m = Map::from([("a", 1), ("b", 2), ("c", 3)]);
    /// // print "a", "b", "c" in arbitrary order.
    /// for key in m.keys() {
    ///     println!("{key}");
    /// }
    /// ```
    ///
    /// # Performance
    /// In the current implementation, iterating over keys takes O(len) time.
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { iter: self.iter() }
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    /// The map cannot be used after calling this. The iterator element type is `K`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let m = Map::from([("a", 1), ("b", 2), ("c", 3)]);
    /// let mut vec: Vec<&str> = m.into_keys().collect();
    /// // The `IntoKeys` iterator produces keys in arbitrary order, so the
    /// // keys must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, ["a", "b", "c"]);
    /// ```
    ///
    /// # Performance
    /// In the current implementation, iterating over keys takes O(len) time.
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V, N> {
        IntoKeys {
            iter: self.into_iter(),
        }
    }
}

/// An iterator over the keys of a `Map`.
///
/// This `struct` is created by the [`keys`][Map::keys] method on [`Map`]. See its
/// documentation for more.
///
/// # Example
/// ```
/// use micromap::Map;
/// let m = Map::from([("a", 1)]);
/// let iter_keys = m.keys();
/// assert_eq!(iter_keys.len(), 1);
/// ```
#[repr(transparent)]
pub struct Keys<'a, K, V> {
    iter: Iter<'a, K, V>,
}

/// An owning iterator over the keys of a `Map`.
///
/// This `struct` is created by the [`into_keys`][Map::into_keys] method on [`Map`].
/// See its documentation for more.
///
/// # Example
/// ```
/// use micromap::Map;
/// let m = Map::from([("a", 1)]);
/// let iter_keys = m.into_keys(); // `.into_keys()` take the ownership
/// // m.len(); // So map cannot be used after calling into_keys
/// assert_eq!(iter_keys.len(), 1);
/// ```
#[repr(transparent)]
pub struct IntoKeys<K, V, const N: usize> {
    iter: IntoIter<K, V, N>,
}

impl<K, V> Clone for Keys<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Keys {
            iter: self.iter.clone(),
        }
    }
}

impl<K: fmt::Debug, V> fmt::Debug for Keys<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.iter.clone().map(|(k, _)| k))
            .finish()
    }
}

impl<K: fmt::Debug, V, const N: usize> fmt::Debug for IntoKeys<K, V, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter.map.keys()).finish()
    }
}

impl<K, V> Default for Keys<'_, K, V> {
    #[inline]
    fn default() -> Self {
        Keys {
            iter: Iter::default(),
        }
    }
}

impl<K, V, const N: usize> Default for IntoKeys<K, V, N> {
    #[inline]
    fn default() -> Self {
        Self {
            iter: IntoIter::default(),
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

impl<K, V, const N: usize> Iterator for IntoKeys<K, V, N> {
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

impl<K, V, const N: usize> ExactSizeIterator for IntoKeys<K, V, N> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> FusedIterator for Keys<'_, K, V> {}

impl<K, V, const N: usize> FusedIterator for IntoKeys<K, V, N> {}

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
