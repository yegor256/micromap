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
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

impl<K: PartialEq + Display, V: Display, const N: usize> Display for Map<K, V, N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <&Self as Debug>::fmt(&self, f)
    }
}

impl<K: PartialEq + Display, V: Display, const N: usize> Debug for Map<K, V, N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut parts = vec![];
        for (k, v) in self.iter() {
            parts.push(format!("{k}: {v}"));
        }
        f.write_str(format!("{{{}}}", parts.join(", ").as_str()).as_str())
    }
}

#[test]
fn debugs_map() {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    m.insert("two", 16);
    assert_eq!("{one: 42, two: 16}", format!("{:?}", m));
}

#[test]
fn displays_map() {
    let mut m: Map<&str, i32, 10> = Map::new();
    m.insert("one", 42);
    m.insert("two", 16);
    assert_eq!("{one: 42, two: 16}", format!("{}", m));
}
