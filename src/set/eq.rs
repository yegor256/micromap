// Copyright (c) 2023-2025 Yegor Bugayenko
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

use crate::Set;

impl<T: PartialEq, const N: usize> PartialEq for Set<T, N> {
    /// Two sets can be compared.
    ///
    /// For example:
    ///
    /// ```
    /// let mut m1: micromap::Set<u8, 10> = micromap::Set::new();
    /// let mut m2: micromap::Set<u8, 10> = micromap::Set::new();
    /// m1.insert(1);
    /// m2.insert(1);
    /// # #[cfg(std)]
    /// assert_eq!(m1, m2);
    /// // two sets with different order of key-value pairs are still equal:
    /// m1.insert(2);
    /// m1.insert(3);
    /// m2.insert(3);
    /// m2.insert(2);
    /// # #[cfg(std)]
    /// assert_eq!(m1, m2);
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map.eq(&other.map)
    }
}

impl<T: Eq, const N: usize> Eq for Set<T, N> {}
