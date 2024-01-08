// Copyright (c) 2023-2024 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
}

impl<T: PartialEq, const N: usize> Iterator for SetIntoIter<T, N> {
    type Item = T;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
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
