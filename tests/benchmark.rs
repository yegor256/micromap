// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

const CAPACITY: usize = 1;

macro_rules! eval {
    ($map:expr, $total:expr, $capacity:expr) => {{
        let mut sum = 0;
        for _ in 0..$total {
            $map.clear();
            let _ = $map.insert(0, 42);
            for i in 1..$capacity - 1 {
                let _ = $map.insert(i as u32, i as i64);
                let v = std::hint::black_box(*$map.get(&(i as u32)).unwrap());
                assert_eq!(v, i as i64);
            }
            for i in 1..$capacity - 1 {
                // for [`indexmap::IndexMap`]`, no logic changes. (or use .swap_remove(key))
                #[allow(deprecated)]
                $map.remove(&(i as u32));
            }
            if $map.iter().find(|(_k, v)| **v == 0).is_some() {
                $map.clear();
            }
            sum += std::hint::black_box($map.iter().find(|(_k, v)| **v == 42).unwrap().1);
        }
        std::hint::black_box(sum)
    }};
}

macro_rules! insert {
    ($name:expr, $ret:expr, $map:expr, $total:expr) => {{
        let start = Instant::now();
        let mut m = $map;
        eval!(m, $total, CAPACITY);
        let e = start.elapsed();
        $ret.insert($name, e);
    }};
}

macro_rules! insert_flurry {
    ($name:expr, $ret:expr, $map:expr, $total:expr) => {{
        let start = Instant::now();
        let m = $map;
        eval!(m.pin(), $total, CAPACITY);
        let e = start.elapsed();
        $ret.insert($name, e);
    }};
}

fn benchmark(total: usize) -> HashMap<&'static str, Duration> {
    let mut ret = HashMap::new();
    insert!(
        "std::collections::HashMap",
        ret,
        HashMap::<u32, i64>::with_capacity(CAPACITY),
        total
    );
    insert!(
        "hashbrown::HashMap",
        ret,
        hashbrown::HashMap::<u32, i64>::new(),
        total
    );
    insert!(
        "rustc_hash::FxHashMap",
        ret,
        rustc_hash::FxHashMap::<u32, i64>::default(),
        total
    );
    insert!(
        "nohash_hasher::BuildNoHashHasher",
        ret,
        HashMap::<u32, i64, nohash_hasher::BuildNoHashHasher<u32>>::with_capacity_and_hasher(
            2,
            nohash_hasher::BuildNoHashHasher::default()
        ),
        total
    );
    insert!(
        "std::collections::BTreeMap",
        ret,
        std::collections::BTreeMap::<u32, i64>::new(),
        total
    );
    insert!(
        "tinymap::array_map::ArrayMap",
        ret,
        tinymap::array_map::ArrayMap::<u32, i64, CAPACITY>::new(),
        total
    );
    insert!(
        "linked_hash_map::LinkedHashMap",
        ret,
        linked_hash_map::LinkedHashMap::<u32, i64>::new(),
        total
    );
    insert!(
        "linear_map::LinearMap",
        ret,
        linear_map::LinearMap::<u32, i64>::new(),
        total
    );
    insert!(
        "indexmap::IndexMap",
        ret,
        indexmap::IndexMap::<u32, i64>::new(),
        total
    );
    insert!(
        "litemap::LiteMap",
        ret,
        litemap::LiteMap::<u32, i64>::new(),
        total
    );
    insert!(
        "heapless::LinearMap",
        ret,
        heapless::LinearMap::<u32, i64, CAPACITY>::new(),
        total
    );
    insert_flurry!(
        "flurry::HashMap",
        ret,
        flurry::HashMap::<u32, i64>::new(),
        total
    );
    insert!(
        "micromap::Map",
        ret,
        micromap::Map::<u32, i64, CAPACITY>::new(),
        total
    );
    ret
}

/// Run it like this from the command line:
///
/// ```text
/// $ cargo test --release --test benchmark -- --include-ignored --nocapture
/// ```
#[test]
#[ignore]
pub fn benchmark_and_print() {
    let times = benchmark(
        #[cfg(debug_assertions)]
        100000,
        #[cfg(not(debug_assertions))]
        10000000,
    );
    let ours = times.get("micromap::Map").unwrap();
    let mut total_gain = 0.0;
    let mut total_loss = 0.0;
    for (m, d) in &times {
        let differential = d.as_nanos() as f64 / ours.as_nanos() as f64;
        println!("{m} -> {:?} ({:.2}x)", d, differential);
        if d == ours {
            continue;
        }
        if d.cmp(ours).is_gt() {
            total_gain += differential;
        } else {
            total_loss += differential;
        }
    }
    println!("Total gain: {:.2}, loss: {:.2}", total_gain, total_loss);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let times = benchmark(args.get(1).unwrap().parse::<usize>().unwrap());
    let mut lines = vec![];
    for (m, d) in &times {
        lines.push(format!("{m}\t{}", d.as_nanos()));
    }
    lines.sort();
    for t in lines {
        println!("{t}");
    }
}
