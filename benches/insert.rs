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

#![feature(test)]

extern crate test;
use micromap::Map;
use test::{black_box, Bencher};

#[bench]
fn insert_same(b: &mut Bencher) {
    let mut m: Map<u64, u64, 64> = Map::new();
    b.iter(|| {
        for i in 0..1000 {
            m.insert(8, 256);
        }
    });
}

#[bench]
fn insert_different(b: &mut Bencher) {
    let mut m: Map<usize, u64, 64> = Map::new();
    b.iter(|| {
        let cap = m.capacity();
        for i in 0..cap {
            m.insert(i, 256);
        }
    });
}

// #[bench]
// fn insert_and_remove(b: &mut Bencher) {
//     let mut m: Map<usize, u64, 64> = Map::new();
//     b.iter(|| {
//         let cap = m.capacity();
//         for i in 0..cap {
//             m.insert(i, 256);
//             m.remove(&i);
//         }
//     });
// }
