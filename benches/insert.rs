// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![feature(test)]

extern crate test;
use micromap::Map;
use test::Bencher;

#[bench]
fn insert_same(b: &mut Bencher) {
    let mut m: Map<u64, u64, 64> = Map::new();
    b.iter(|| {
        for i in 0..1000 {
            m.insert(8, i);
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

#[bench]
fn insert_and_remove(b: &mut Bencher) {
    let mut m: Map<usize, u64, 64> = Map::new();
    b.iter(|| {
        let cap = m.capacity();
        for i in 0..cap {
            m.insert(i, 256);
            m.remove(&i);
        }
    });
}
