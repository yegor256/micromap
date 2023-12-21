// Copyright (c) 2023 Yegor Bugayenko
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

use crate::{Map, Set};

impl<T: PartialEq, const N: usize> Default for Set<T, N> {
    /// Make a default empty [`Set`].
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq, const N: usize> Set<T, N> {
    /// Make it.
    ///
    /// The size of the set is defined by the generic argument. For example,
    /// this is how you make a set of four key-values pairs:
    #[inline]
    #[must_use]
    #[allow(clippy::uninit_assumed_init)]
    pub const fn new() -> Self {
        Self {
            map: Map::<T, (), N>::new(),
        }
    }
}
