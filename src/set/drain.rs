// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Set;
use core::iter::FusedIterator;

#[repr(transparent)]
pub struct Drain<'a, T> {
    iter: crate::map::drain::Drain<'a, T, ()>,
}

impl<T, const N: usize> Set<T, N> {
    /// Clears the set, returning all elements as an iterator. Keeps the allocated
    /// memory for reuse.
    ///
    /// If the returned iterator is dropped before being fully consumed, it drops the
    /// remaining elements. The returned iterator keeps a mutable borrow on the set
    /// to optimize its implementation.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set = Set::from([1, 2, 3]);
    /// assert!(!set.is_empty());
    /// // print 1, 2, 3 in an arbitrary order
    /// for i in set.drain() {
    ///     println!("{i}");
    /// }
    /// assert!(set.is_empty());
    /// ```
    pub fn drain(&mut self) -> Drain<'_, T> {
        Drain {
            iter: self.map.drain(),
        }
    }
}

impl<K> Iterator for Drain<'_, K> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, ())| k)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.iter.len(), Some(self.iter.len()))
    }
}

impl<K> ExactSizeIterator for Drain<'_, K> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K> FusedIterator for Drain<'_, K> {}
