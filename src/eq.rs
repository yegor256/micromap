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

use crate::Map;

impl<K: Clone + PartialEq, V: Clone + PartialEq, const N: usize> PartialEq for Map<K, V, N> {
    /// Two maps can be compared.
    ///
    /// For example:
    ///
    /// ```
    /// let mut m1: micromap::Map<u8, i32, 10> = micromap::Map::new();
    /// let mut m2: micromap::Map<u8, i32, 10> = micromap::Map::new();
    /// m1.insert(1, 42);
    /// m2.insert(1, 42);
    /// assert_eq!(m1, m2);
    /// // two maps with different order of key-value pairs are still equal:
    /// m1.insert(2, 1);
    /// m1.insert(3, 16);
    /// m2.insert(3, 16);
    /// m2.insert(2, 1);
    /// assert_eq!(m1, m2);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        return self.len() == other.len() && self.iter().all(|(k, v)| other.get(k) == Some(v));
    }
}

impl<K: Clone + Eq, V: Clone + Eq, const N: usize> Eq for Map<K, V, N> {}

#[test]
fn compares_two_maps() {
    let mut m1: Map<&str, i32, 10> = Map::new();
    m1.insert("first", 42);
    let mut m2: Map<&str, i32, 10> = Map::new();
    m2.insert("first", 42);
    assert!(m1.eq(&m2));
}
