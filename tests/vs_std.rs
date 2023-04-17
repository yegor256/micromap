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

// In order to run this single test:
// $ cargo test --test vs_std -- --nocapture

use std::collections::HashMap;
use std::time::Instant;

const CAPACITY: usize = 10;

pub fn with_std(total: usize) -> i64 {
    let mut sum = 0;
    for _ in 0..total {
        let mut m = HashMap::with_capacity(CAPACITY);
        m.insert(0, 42);
        for i in 1..CAPACITY - 1 {
            m.insert(i, i as i64);
            m.get(&i);
            m.remove(&i);
        }
        sum += m.into_iter().find(|(_k, v)| *v == 42).unwrap().1
    }
    std::hint::black_box(sum)
}

pub fn with_micromap(total: usize) -> i64 {
    let mut sum = 0;
    for _ in 0..total {
        let mut m: micromap::Map<usize, i64, CAPACITY> = micromap::Map::new();
        m.insert(0, 42);
        for i in 1..CAPACITY - 1 {
            m.insert(i, i as i64);
            m.get(&i);
            m.remove(&i);
        }
        sum += m.into_iter().find(|(_k, v)| *v == 42).unwrap().1
    }
    std::hint::black_box(sum)
}

#[test]
pub fn main() {
    let total = 100000;
    let start1 = Instant::now();
    let s1 = with_std(total);
    let e1 = start1.elapsed();
    println!("hashmap: {:?}", e1);
    let start2 = Instant::now();
    let s2 = with_micromap(total);
    let e2 = start2.elapsed();
    println!("micromap: {:?}", e2);
    println!("gain: {:.2}x", e1.as_nanos() as f64 / e2.as_nanos() as f64);
    assert_eq!(s1, s2);
}
