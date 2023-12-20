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

use crate::{IntoKeys, Keys, Map};

impl<K: PartialEq, V, const N: usize> Map<K, V, N> {
    /// An iterator visiting all keys in arbitrary order.
    #[inline]
    pub const fn keys(&self) -> Keys<'_, K, V, N> {
        Keys { iter: self.iter() }
    }

    /// Consuming iterator visiting all keys in arbitrary order.
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V, N> {
        IntoKeys {
            iter: self.into_iter(),
        }
    }
}

impl<'a, K: PartialEq, V, const N: usize> Iterator for Keys<'a, K, V, N> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| p.0)
    }
}

impl<K: PartialEq, V, const N: usize> Iterator for IntoKeys<K, V, N> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<K> {
        self.iter.next().map(|p| p.0)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn iterate_keys() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("foo".to_string(), 0);
        m.insert("bar".to_string(), 0);
        assert_eq!(m.keys().collect::<Vec<_>>(), [&"foo", &"bar"]);
    }

    #[test]
    fn iterate_into_keys() {
        let mut m: Map<String, i32, 10> = Map::new();
        m.insert("foo".to_string(), 0);
        m.insert("bar".to_string(), 0);
        assert_eq!(
            m.into_keys().collect::<Vec<_>>(),
            ["foo".to_string(), "bar".to_string()]
        );
    }
}
