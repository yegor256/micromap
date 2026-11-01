// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use micromap::Map;
use proptest::collection::vec;
use proptest::prelude::*;
use std::collections::HashMap;

const CAPACITY: usize = 16;

#[derive(Debug, Clone)]
enum Op {
    Insert(u8, u32),
    Remove(u8),
    Get(u8),
    Clear,
}

fn op_strategy() -> impl Strategy<Value = Op> {
    prop_oneof![
        8 => (0u8..32u8, any::<u32>()).prop_map(|(k, v)| Op::Insert(k, v)),
        4 => (0u8..32u8).prop_map(Op::Remove),
        4 => (0u8..32u8).prop_map(Op::Get),
        1 => Just(Op::Clear),
    ]
}

proptest! {
    /// For any sequence of operations within capacity, `micromap::Map` must
    /// stay observably identical to `std::collections::HashMap`.
    #[test]
    fn behaves_like_hashmap(ops in vec(op_strategy(), 0..200)) {
        let mut m: Map<u8, u32, CAPACITY> = Map::new();
        let mut h: HashMap<u8, u32> = HashMap::new();
        for op in ops {
            match op {
                Op::Insert(k, v) => {
                    if h.contains_key(&k) || h.len() < CAPACITY {
                        prop_assert_eq!(m.insert(k, v), h.insert(k, v));
                    }
                }
                Op::Remove(k) => {
                    prop_assert_eq!(m.remove(&k), h.remove(&k));
                }
                Op::Get(k) => {
                    prop_assert_eq!(m.get(&k), h.get(&k));
                }
                Op::Clear => {
                    m.clear();
                    h.clear();
                }
            }
            prop_assert_eq!(m.len(), h.len());
            prop_assert_eq!(m.is_empty(), h.is_empty());
            for (k, v) in &h {
                prop_assert_eq!(m.get(k), Some(v));
                prop_assert!(m.contains_key(k));
            }
        }
    }

    /// `checked_insert` must never panic, regardless of capacity pressure.
    #[test]
    fn checked_insert_never_panics(pairs in vec((any::<u8>(), any::<u32>()), 0..200)) {
        let mut m: Map<u8, u32, 8> = Map::new();
        for (k, v) in pairs {
            let _ = m.checked_insert(k, v);
            prop_assert!(m.len() <= 8);
        }
    }
}
