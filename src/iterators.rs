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

use crate::{IntoIter, Iter, Map};

impl<'a, K: Clone, V: Clone, const N: usize> Iterator for Iter<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.next {
            if let Present(p) = &self.pairs[self.pos] {
                self.pos += 1;
                return Some((&p.0, &p.1));
            }
            self.pos += 1;
        }
        None
    }
}

impl<'a, K: Clone, V: Clone, const N: usize> Iterator for IntoIter<'a, K, V, N> {
    type Item = (K, V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.next {
            if self.pairs[self.pos].is_some() {
                let pair = self.pairs[self.pos].clone().unwrap();
                self.pos += 1;
                return Some(pair);
            }
            self.pos += 1;
        }
        None
    }
}

impl<'a, K: Copy + PartialEq, V: Copy, const N: usize> IntoIterator for &'a Map<K, V, N> {
    type Item = (K, V);
    type IntoIter = IntoIter<'a, K, V, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            next: self.next,
            pos: 0,
            pairs: &self.pairs,
        }
    }
}

use crate::Pair::Present;
#[cfg(test)]
use anyhow::Result;

#[test]
fn empty_iterator() -> Result<()> {
    let m: Map<u32, u32, 4> = Map::new();
    assert!(m.into_iter().next().is_none());
    Ok(())
}

#[test]
fn insert_and_jump_over_next() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("foo", 42);
    let mut iter = m.into_iter();
    assert_eq!(42, iter.next().unwrap().1);
    assert!(iter.next().is_none());
    Ok(())
}

#[test]
fn insert_and_iterate() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    m.insert("two", 16);
    let mut sum = 0;
    for (_k, v) in m.iter() {
        sum += v;
    }
    assert_eq!(58, sum);
    Ok(())
}

#[test]
fn insert_and_into_iterate() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    m.insert("two", 16);
    let mut sum = 0;
    for (_k, v) in m.into_iter() {
        sum += v;
    }
    assert_eq!(58, sum);
    Ok(())
}
