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

impl<K: Clone + PartialEq, V: Clone, const N: usize> Clone for Map<K, V, N> {
    fn clone(&self) -> Self {
        let mut m: Self = Self::new();
        for (k, v) in self.iter() {
            m.insert(k.clone(), v.clone());
        }
        m
    }
}

#[test]
fn map_can_be_cloned() {
    let mut m: Map<u8, u8, 16> = Map::new();
    m.insert(0, 42);
    assert_eq!(42, *m.clone().get(&0).unwrap());
}

#[test]
fn empty_map_can_be_cloned() {
    let m: Map<u8, u8, 16> = Map::new();
    assert!(m.clone().is_empty());
}
