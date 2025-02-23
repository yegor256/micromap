// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![feature(test)]

extern crate test;
use micromap::Map;
use test::Bencher;

#[bench]
fn straight_length(b: &mut Bencher) {
    let mut m: Map<u64, u64, 64> = Map::new();
    for i in 0..1000 {
        m.insert(8, i);
    }
    b.iter(|| {
        for _ in 0..1000 {
            test::black_box(m.len());
        }
    });
}

#[bench]
fn fragmented_length(b: &mut Bencher) {
    let mut m: Map<u64, u64, 64> = Map::new();
    for i in 0..1000 {
        m.insert(8, i);
    }
    for i in 0..1000 {
        if i % 7 != 0 {
            m.remove(&i);
        }
    }
    b.iter(|| {
        for _ in 0..1000 {
            test::black_box(m.len());
        }
    });
}
