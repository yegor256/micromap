// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::SetDrain;
use core::iter::FusedIterator;

impl<'a, K: PartialEq> Iterator for SetDrain<'a, K> {
    type Item = K;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, ())| k)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.iter.len(), Some(self.iter.len()))
    }
}

impl<'a, K: PartialEq> ExactSizeIterator for SetDrain<'a, K> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: PartialEq> FusedIterator for SetDrain<'a, K> {}
