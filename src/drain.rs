// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Drain;
use core::iter::FusedIterator;

impl<'a, K, V> Drop for Drain<'a, K, V> {
    fn drop(&mut self) {
        for pair in &mut self.iter {
            unsafe { pair.assume_init_drop() };
        }
    }
}

impl<'a, K: PartialEq, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| unsafe { p.assume_init_read() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.iter.len(), Some(self.iter.len()))
    }
}

impl<'a, K: PartialEq, V> ExactSizeIterator for Drain<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: PartialEq, V> FusedIterator for Drain<'a, K, V> {}
