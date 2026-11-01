// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Set;
use core::iter::FusedIterator;

/// Iterator over the [`Set`].
#[repr(transparent)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T> {
    iter: crate::map::keys::Keys<'a, T, ()>,
}

/// Into-iterator over the [`Set`].
#[repr(transparent)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IntoIter<T, const N: usize> {
    iter: crate::map::keys::IntoKeys<T, (), N>,
}

impl<T, const N: usize> Set<T, N> {
    /// An iterator visiting all elements in arbitrary order. The iterator
    /// element type is `&'a T`.
    ///
    /// # Examples
    /// ```
    /// use micromap::Set;
    /// let mut set: Set<_, 3> = Set::new();
    /// set.insert("a");
    /// set.insert("b");
    /// // Will print in an arbitrary order.
    /// for x in set.iter() {
    ///     println!("{x}");
    /// }
    /// ```
    ///
    /// # Performance
    /// In the current implementation, iterating over set takes O(len) time.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.map.keys(),
        }
    }
}

impl<T> Clone for Iter<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Set<T, N> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, const N: usize> IntoIterator for Set<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.map.into_keys(),
        }
    }
}

impl<'a, T: 'a> ExactSizeIterator for Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T: 'a> FusedIterator for Iter<'a, T> {}

impl<T, const N: usize> FusedIterator for IntoIter<T, N> {}
