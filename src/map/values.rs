// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::iterators::slice_iter;
use super::iterators::{IntoIter, Iter, IterMut};
use super::Map;
use core::{fmt, iter::FusedIterator};

impl<K, V, const N: usize> Map<K, V, N> {
    /// An iterator visiting all values in arbitrary order. The iterator element
    /// type is `&'a V`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let m = Map::from([("a", 1), ("b", 2), ("c", 3)]);
    /// // print "a", "b", "c" in arbitrary order.
    /// for val in m.values() {
    ///     println!("{val}");
    /// }
    /// ```
    ///
    /// # Performance
    /// In the current implementation, iterating over keys takes O(len) time.
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values { iter: self.iter() }
    }

    /// An iterator visiting all values mutably in arbitrary order. The iterator
    /// element type is `&'a mut V`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut m = Map::from([("a", 1), ("b", 2), ("c", 3)]);
    /// for val in m.values_mut() {
    ///     *val = *val + 10;
    /// }
    /// for val in m.values() {
    ///     println!("{val}");
    /// }
    /// ```
    ///
    /// # Performance
    /// In the current implementation, iterating over keys takes O(len) time.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            iter: self.iter_mut(),
        }
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    /// The map cannot be used after calling this. The iterator element type is `V`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let m = Map::from([("a", 1), ("b", 2), ("c", 3)]);
    /// let mut vec: Vec<i32> = m.into_values().collect();
    /// // The `IntoValues` iterator produces values in arbitrary order, so the
    /// // values must be sorted to test them against a sorted array.
    /// vec.sort_unstable();
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    ///
    /// # Performance
    /// In the current implementation, iterating over keys takes O(len) time.
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V, N> {
        IntoValues {
            iter: self.into_iter(),
        }
    }
}

/// An iterator over the values of a `Map`.
///
/// This `struct` is created by the [`values`][Map::values] method on [`Map`]. See
/// its documentation for more.
///
/// # Example
/// ```
/// use micromap::Map;
/// let m = Map::from([("a", 1)]);
/// let iter_values = m.values();
/// assert_eq!(iter_values.len(), 1);
/// ```
#[repr(transparent)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Values<'a, K, V> {
    iter: Iter<'a, K, V>,
}

/// A mutable iterator over the values of a `Map`.
///
/// This `struct` is created by the [`values_mut`][Map::values_mut] method on [`Map`].
/// See its documentation for more.
///
/// # Example
/// ```
/// use micromap::Map;
/// let mut m = Map::from([("a", 1), ("b", 2)]);
/// let iter_values = m.values_mut();
/// iter_values.for_each(|v| *v *= 2);
/// assert_eq!(m, Map::from([("a", 2), ("b", 4)]));
/// ```
#[repr(transparent)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ValuesMut<'a, K, V> {
    iter: IterMut<'a, K, V>,
}

/// An owning iterator over the values of a `Map`.
///
/// This `struct` is created by the [`into_values`][Map::into_values] method on
/// [`Map`]. See its documentation for more.
///
/// # Example
/// ```
/// use micromap::Map;
/// let m = Map::from([("a", 1), ("b", 2)]);
/// let iter_values = m.into_values();
/// let mut vec: Vec<_> = iter_values.collect();
/// vec.sort_unstable();
/// assert_eq!(vec, [1, 2]);
/// ```
#[repr(transparent)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IntoValues<K, V, const N: usize> {
    iter: IntoIter<K, V, N>,
}

impl<K, V> Clone for Values<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<K, V> Default for Values<'_, K, V> {
    #[inline]
    fn default() -> Self {
        Self {
            iter: Iter::default(),
        }
    }
}

impl<K, V> Default for ValuesMut<'_, K, V> {
    #[inline]
    fn default() -> Self {
        Self {
            iter: IterMut::default(),
        }
    }
}

impl<K, V, const N: usize> Default for IntoValues<K, V, N> {
    #[inline]
    fn default() -> Self {
        Self {
            iter: IntoIter::default(),
        }
    }
}

impl<K, V: fmt::Debug> fmt::Debug for Values<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.iter.clone().map(|(_, v)| v))
            .finish()
    }
}

impl<K, V: fmt::Debug> fmt::Debug for ValuesMut<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(slice_iter(self.iter.iter.as_slice()).map(|(_, v)| v))
            .finish()
    }
}

impl<K, V: fmt::Debug, const N: usize> fmt::Debug for IntoValues<K, V, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter.map.values()).finish()
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| p.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| p.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K, V, const N: usize> Iterator for IntoValues<K, V, N> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<V> {
        self.iter.next().map(|p| p.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K, V> ExactSizeIterator for Values<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V, const N: usize> ExactSizeIterator for IntoValues<K, V, N> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> FusedIterator for Values<'_, K, V> {}
impl<K, V> FusedIterator for ValuesMut<'_, K, V> {}
impl<K: PartialEq, V, const N: usize> FusedIterator for IntoValues<K, V, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_values() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 42);
        m.insert("two".to_string(), 16);
        let it = m.values();
        assert_eq!(it.len(), 2);
        assert_eq!(58, it.sum::<i32>());
    }

    #[test]
    fn iterate_values_mut() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 42);
        m.insert("two".to_string(), 16);
        let it_mut = m.values_mut();
        assert_eq!(it_mut.len(), 2);
        assert_eq!(it_mut.len(), it_mut.size_hint().0);
        assert_eq!(it_mut.len(), it_mut.size_hint().1.unwrap());
        it_mut.for_each(|v| *v *= 2);
        assert_eq!(116, m.values().sum::<i32>());
    }

    #[test]
    fn iterate_values_with_blanks() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 1);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        m.remove("two");
        assert_eq!(m.values().collect::<Vec<_>>(), [&1, &5]);
    }

    #[test]
    fn into_values_drop() {
        use std::rc::Rc;
        let mut m: Map<i32, Rc<()>, 8> = Map::new();
        let v = Rc::new(());
        for i in 0..8 {
            m.insert(i, Rc::clone(&v));
        }
        assert_eq!(9, Rc::strong_count(&v));
        let mut values = m.into_values();
        assert!(values.next().is_some());
        assert_eq!(values.len(), 7);
        assert!(values.next().is_some());
        assert_eq!(values.len(), values.size_hint().0);
        drop(values);
        assert_eq!(1, Rc::strong_count(&v));
    }
}
