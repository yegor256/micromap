// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;
use micromap::Map;
use std::collections::{HashMap, HashSet};

use micromap_fuzz::{apply_op, FuzzInput, MAX_CAPACITY, MAX_OPS};

fuzz_target!(|data: FuzzInput| {
    let mut map = Map::<u8, u8, MAX_CAPACITY>::new();
    let mut shadow = HashMap::<u8, u8>::new();

    for op in data.ops.iter().take(MAX_OPS) {
        apply_op(&mut map, &mut shadow, op);
    }

    let lhs: HashSet<_> = map.iter().map(|(k, v)| (*k, *v)).collect();
    let rhs: HashSet<_> = shadow.iter().map(|(k, v)| (*k, *v)).collect();
    assert_eq!(map.len(), shadow.len(), "final length mismatch");
    assert_eq!(lhs, rhs, "final content mismatch");
});

