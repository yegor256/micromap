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
use std::borrow::Borrow;
use std::ops::{Index, IndexMut};

impl<K: Copy + Eq + Borrow<Q>, Q: Eq + ?Sized, V: Clone, const N: usize> Index<&Q>
    for Map<K, V, N>
{
    type Output = V;

    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl<K: Copy + Eq + Borrow<Q>, Q: Eq + ?Sized, V: Clone, const N: usize> IndexMut<&Q>
    for Map<K, V, N>
{
    #[inline]
    fn index_mut(&mut self, key: &Q) -> &mut V {
        self.get_mut(key).expect("no entry found for key")
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn index() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("first", 42);
    assert_eq!(m["first"], 42);
    Ok(())
}

#[test]
fn index_mut() -> Result<()> {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("first", 42);
    m["first"] += 10;
    assert_eq!(m["first"], 52);
    Ok(())
}

#[test]
#[should_panic]
fn wrong_index() -> () {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("first", 42);
    assert_eq!(m["second"], 42);
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct Container {
    pub t: i32,
}

#[cfg(test)]
impl Borrow<i32> for Container {
    fn borrow(&self) -> &i32 {
        &self.t
    }
}

#[test]
fn index_by_borrow() -> Result<()> {
    let mut m: Map<Container, i32, 10> = Map::new();
    m.insert(Container { t: 10 }, 42);
    assert_eq!(m[&10], 42);
    Ok(())
}
