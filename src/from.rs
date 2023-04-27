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

impl<K: Clone + PartialEq, V: Clone, const N: usize> FromIterator<(K, V)> for Map<K, V, N> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut m: Self = Self::new();
        for (k, v) in iter {
            m.insert(k, v);
        }
        m
    }
}

impl<K: Clone + PartialEq, V: Clone, const N: usize> From<[(K, V); N]> for Map<K, V, N> {
    #[inline]
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

#[cfg(test)]
const TEST_ARRAY: [(i32, &str); 5] = [(1, "sun"), (2, "mon"), (3, "tue"), (4, "wed"), (5, "thu")];

#[test]
fn from_iter() {
    let vec = Vec::from(TEST_ARRAY);
    let m: Map<i32, &str, 10> = Map::from_iter(vec);
    assert_eq!(m.len(), 5);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn from_larger_iter() {
    let vec = Vec::from(TEST_ARRAY);
    let _m: Map<i32, &str, 1> = Map::from_iter(vec);
}

#[test]
fn from_array() {
    let m = Map::from(TEST_ARRAY);
    assert_eq!(m.len(), 5);
}

#[test]
fn array_into_map() {
    let m: Map<i32, &str, 5> = TEST_ARRAY.into();
    assert_eq!(m.len(), 5);
}

#[test]
fn from_with_duplicates() {
    let arr = [(1, "sun"), (2, "mon"), (3, "tue"), (1, "wed"), (2, "thu")];
    let m = Map::from(arr);
    assert_eq!(m.len(), 3);
    assert_eq!(m[&2], "thu");
}
