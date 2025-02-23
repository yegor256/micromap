// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::set::{Set, SetIntoIter, SetIter};
use core::iter::FusedIterator;

impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Make an iterator over all pairs.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> SetIter<T> {
        SetIter {
            iter: self.map.keys(),
        }
    }
}

impl<'a, T> Iterator for SetIter<'a, T> {
    type Item = &'a T;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T: PartialEq, const N: usize> Iterator for SetIntoIter<T, N> {
    type Item = T;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T: PartialEq, const N: usize> IntoIterator for &'a Set<T, N> {
    type Item = &'a T;
    type IntoIter = SetIter<'a, T>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: PartialEq, const N: usize> IntoIterator for Set<T, N> {
    type Item = T;
    type IntoIter = SetIntoIter<T, N>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        SetIntoIter {
            iter: self.map.into_keys(),
        }
    }
}

impl<'a, T> ExactSizeIterator for SetIter<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T: PartialEq, const N: usize> ExactSizeIterator for SetIntoIter<T, N> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T> FusedIterator for SetIter<'a, T> {}

impl<T: PartialEq, const N: usize> FusedIterator for SetIntoIter<T, N> {}
