// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Map;
use core::{fmt, iter::FusedIterator, mem::MaybeUninit};

impl<K, V, const N: usize> Map<K, V, N> {
    /// An iterator visiting all key-value pairs in _non-deterministic order_.
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// If no mutable borrow modifications are made to the map before the next
    /// iteration, the order of iteration will not change.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let map = Map::from([("a", 1), ("b", 2), ("c", 3)]);
    /// for (k, v) in &map { // Implicit call as `map.iter()`
    ///     println!("key: {k} val: {v}");
    /// }
    /// ```
    ///
    /// # Performance
    /// In the current implementation, iterating over map takes `O(len)` time.
    /// The average complexity is `O(len/2)`.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            iter: self.pairs[..self.len].as_ref().iter(),
        }
    }

    /// An iterator visiting all key-value pairs in _arbitrary order_, with
    /// mutable references to the values. The iterator element type
    /// is `(&'a K, &'a mut V)`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Map;
    /// let mut map = Map::from([("a", 1), ("b", 2), ("c", 3)]);
    /// // Update all values
    /// for (_, val) in &mut map { // Implicit call as `map.iter_mut()`
    ///     *val *= 2;
    /// }
    /// for (k, v) in &map { // Implicit call as `map.iter()`
    ///     println!("key: {k} val: {v}"); // vals are now 2, 4, 6
    /// }
    /// ```
    ///
    /// # Performance
    /// In the current implementation, iterating over map takes `O(len)` time.
    /// The average complexity is `O(len/2)`.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            iter: self.pairs[..self.len].as_mut().iter_mut(),
        }
    }
}

/// An iterator over the entries of a `HashMap`.
///
/// This `struct` is created by the [`iter`][`Map::iter`] method on [`Map`].
/// See its documentation for more.
///
/// # Example
/// ```
/// use micromap::Map;
/// let map = Map::from([("a", 1), ("b", 2)]);
/// let mut iter = map.iter();
/// assert_eq!(iter.len(), 2);
/// assert_eq!(iter.next(), Some((&"a", &1)));
/// assert_eq!(iter.len(), 1);
/// assert_eq!(iter.next(), Some((&"b", &2)));
/// assert_eq!(iter.len(), 0);
/// assert_eq!(iter.next(), None);
/// ```
#[repr(transparent)]
pub struct Iter<'a, K, V> {
    iter: core::slice::Iter<'a, MaybeUninit<(K, V)>>,
}

/// A mutable iterator over the entries of a `Map`.
///
/// This `struct` is created by the [`iter_mut`][`Map::iter_mut`] method
/// on [`Map`]. See its documentation for more.
///
/// # Example
/// ```
/// use micromap::Map;
/// let mut map = Map::from([("a", 1), ("b", 2)]);
/// let mut iter = map.iter_mut();
/// iter.next().map(|(_, v)| *v *= 2); // ("a", 2)
/// iter.next().map(|(_, v)| *v += 1); // ("b", 3)
/// assert_eq!(iter.len(), 0);
/// assert_eq!(iter.next(), None);
/// assert_eq!(map, Map::from([("a", 2), ("b", 3)]));
/// ```
#[repr(transparent)]
pub struct IterMut<'a, K, V> {
    iter: core::slice::IterMut<'a, MaybeUninit<(K, V)>>,
}

/// An owning iterator over the entries of a `Map`.
///
/// This `struct` is created by the [`into_iter`][`IntoIterator::into_iter`]
/// method on [`Map`] (provided by the [`IntoIterator`] trait). See its
/// documentation for more.
///
/// # Example
/// ```
/// use micromap::Map;
/// let map = Map::from([("a", 1)]);
/// let _iter = map.into_iter(); // consumed map
/// let mut map = Map::from([('b', 2), ('c', 3)]);
/// for (k, v) in map { // Implicit call as `map.into_iter()`
///     println!("key: {k} val: {v}");
/// }
/// // assert_eq!(map.len(), 2); // `into_iter()` takes ownership, so can not do this
/// ```
#[repr(transparent)]
pub struct IntoIter<K, V, const N: usize> {
    map: Map<K, V, N>,
}

/// Utility function for implementing Debug trait for iterators (whose inner is a slice).
#[inline]
pub fn slice_iter<K, V>(slice: &[MaybeUninit<(K, V)>]) -> impl Iterator<Item = (&K, &V)> {
    slice.iter().map(|may| unsafe {
        let (k, v) = may.assume_init_ref();
        (k, v)
    })
}

impl<K, V> Clone for Iter<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for Iter<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(slice_iter(self.iter.as_slice()))
            .finish()
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for IterMut<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(slice_iter(self.iter.as_slice()))
            .finish()
    }
}

impl<K: fmt::Debug, V: fmt::Debug, const N: usize> fmt::Debug for IntoIter<K, V, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.map.iter()).finish()
    }
}

impl<K, V> Default for Iter<'_, K, V> {
    #[inline]
    fn default() -> Self {
        Self { iter: [].iter() }
    }
}

impl<K, V> Default for IterMut<'_, K, V> {
    #[inline]
    fn default() -> Self {
        Self {
            iter: [].iter_mut(),
        }
    }
}

impl<K, V, const N: usize> Default for IntoIter<K, V, N> {
    #[inline]
    fn default() -> Self {
        Self {
            map: Map::default(),
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| {
            let p = unsafe { p.assume_init_ref() };
            (&p.0, &p.1)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| {
            let p = unsafe { p.assume_init_mut() };
            (&p.0, &mut p.1)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.len()
    }
}

impl<K, V, const N: usize> Iterator for IntoIter<K, V, N> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.map.len > 0 {
            self.map.len -= 1;
            Some(unsafe { self.map.item_read(self.map.len) })
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.map.len, Some(self.map.len))
    }

    #[inline]
    fn count(self) -> usize {
        self.map.len()
    }
}

impl<'a, K, V, const N: usize> IntoIterator for &'a Map<K, V, N> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, const N: usize> IntoIterator for &'a mut Map<K, V, N> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, const N: usize> IntoIterator for Map<K, V, N> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { map: self }
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V, const N: usize> ExactSizeIterator for IntoIter<K, V, N> {
    fn len(&self) -> usize {
        self.map.len
    }
}

impl<K, V> FusedIterator for Iter<'_, K, V> {}

impl<K, V> FusedIterator for IterMut<'_, K, V> {}

impl<K, V, const N: usize> FusedIterator for IntoIter<K, V, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_iterator() {
        let m: Map<u32, u32, 4> = Map::new();
        assert!(m.into_iter().next().is_none());
    }

    #[test]
    fn insert_and_jump_over_next() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("foo".to_string(), 42);
        let mut iter = m.into_iter();
        assert_eq!(42, iter.next().unwrap().1);
        assert!(iter.next().is_none());
    }

    #[test]
    fn insert_and_iterate() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 42);
        m.insert("two".to_string(), 16);
        let mut sum = 0;
        for (_k, v) in m.iter() {
            sum += v;
        }
        assert_eq!(58, sum);
    }

    #[test]
    fn insert_and_into_iterate() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 42);
        m.insert("two".to_string(), 16);
        let mut sum = 0;
        for p in &m {
            sum += p.1;
        }
        assert_eq!(58, sum);
    }

    #[test]
    fn iterate_with_blanks() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 1);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        m.remove("two");
        let mut sum = 0;
        for (_k, v) in m.iter() {
            sum += v;
        }
        assert_eq!(6, sum);
    }

    #[test]
    fn debug_trait_for_iter() {
        let map = Map::from([('x', 120), ('y', 121), ('z', 122)]);
        let it = map.iter();
        // use Debug trait by `format!` macro
        let debug_output = format!("{:?}", it);
        assert!(debug_output.contains("('x', 120)"));
        assert!(debug_output.contains("('y', 121)"));
        assert!(debug_output.contains("('z', 122)"));
    }

    #[test]
    fn debug_trait_for_iter_mut() {
        let mut map = Map::from([('x', 120), ('y', 121), ('z', 122)]);
        let it = map.iter_mut();
        // use Debug trait by `format!` macro
        let debug_output = format!("{:?}", it);
        assert!(debug_output.contains("('x', 120)"));
        assert!(debug_output.contains("('y', 121)"));
        assert!(debug_output.contains("('z', 122)"));
    }

    #[test]
    fn debug_trait_for_into_iter() {
        let map = Map::from([('x', 120), ('y', 121), ('z', 122)]);
        let it = map.into_iter();
        // use Debug trait by `format!` macro
        let debug_output = format!("{:?}", it);
        assert!(debug_output.contains("('x', 120)"));
        assert!(debug_output.contains("('y', 121)"));
        assert!(debug_output.contains("('z', 122)"));
    }

    #[test]
    fn into_iterate_with_blanks() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 1);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        m.remove("two");
        let mut sum = 0;
        for (_k, v) in m {
            sum += v;
        }
        assert_eq!(6, sum);
    }

    #[test]
    fn change_with_iter_mut() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 2);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        for (_k, v) in m.iter_mut() {
            *v *= 2;
        }
        let sum = m.iter().map(|p| p.1).sum::<i32>();
        assert_eq!(20, sum);
    }

    #[test]
    fn iter_mut_with_blanks() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 1);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        assert_eq!(m.iter_mut().count(), 3);
        m.remove("two");
        assert_eq!(m.iter_mut().count(), 2);
        assert_eq!(m.iter_mut().last().unwrap().1, &5);
    }

    #[test]
    fn iter_mut_key_is_not_mut() {
        let mut m: Map<u8, i32, 10> = Map::new();
        m.insert(1, 10);
        m.insert(2, 20);
        for (k, v) in m.iter_mut() {
            // *k += 1; // this will not compile, because `k` is &u8
            *v += *k as i32; // v is &mut i32
        }
    }

    #[test]
    fn into_iter_mut() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("one".to_string(), 2);
        m.insert("two".to_string(), 3);
        m.insert("three".to_string(), 5);
        for (_k, v) in &mut m {
            *v *= 2;
        }
        let sum = m.iter().map(|p| p.1).sum::<i32>();
        assert_eq!(20, sum);
    }

    #[test]
    fn into_iter_drop() {
        use std::rc::Rc;
        let mut m: Map<i32, Rc<()>, 8> = Map::new();
        let v = Rc::new(());
        let n = 8;
        for i in 0..n {
            m.insert(i, Rc::clone(&v));
        }
        assert_eq!(Rc::strong_count(&v), (n + 1) as usize);
        let _p = m.into_iter().nth(3);
        assert_eq!(Rc::strong_count(&v), 2); // v & p
    }

    #[test]
    fn iter_size_hint() {
        let mut m: Map<char, u32, 4> = Map::new();
        m.insert('a', 97);
        m.insert('c', 99);
        let it = m.iter();
        assert_eq!(it.len(), 2);
        let mut it_mut = m.iter_mut();
        assert!(it_mut.next().is_some());
        assert_eq!(it_mut.len(), 1);
        assert_eq!(it_mut.len(), it_mut.size_hint().0);
        let mut it_into = m.into_iter();
        assert!(it_into.next().is_some());
        assert!(it_into.next().is_some());
        assert!(it_into.next().is_none());
        assert!(it_into.next().is_none());
        assert_eq!(it_into.len(), 0);
    }

    #[test]
    fn iter_series_default() {
        let _i = Iter::<String, u32>::default();
        let _i = IterMut::<String, u32>::default();
        let _i = IntoIter::<String, u32, 3>::default();
    }
}
