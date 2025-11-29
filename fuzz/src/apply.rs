// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};

use micromap::Map;

use crate::{input::Op, MAX_CAPACITY};

pub fn apply_op(
    map: &mut Map<u8, u8, MAX_CAPACITY>,
    shadow: &mut HashMap<u8, u8>,
    op: &Op,
) {
    match *op {
        Op::Insert { key, value } => {
            match map.checked_insert(key, value) {
                Some(Some(old)) => {
                    let prev = shadow.insert(key, value);
                    assert_eq!(prev, Some(old), "shadow must replace the same value");
                }
                Some(None) => {
                    let prev = shadow.insert(key, value);
                    assert!(prev.is_none(), "shadow unexpectedly replaced a value");
                }
                None => {
                    // Map is full and the key was absent. Shadow should mirror this state.
                    assert!(!shadow.contains_key(&key));
                    assert_eq!(map.len(), MAX_CAPACITY);
                    assert_eq!(shadow.len(), MAX_CAPACITY);
                }
            }
        }
        Op::Remove { key } => {
            let left = map.remove(&key);
            let right = shadow.remove(&key);
            assert_eq!(left, right, "remove mismatch for key {key}");
        }
        Op::Get { key } => {
            let left = map.get(&key);
            let right = shadow.get(&key);
            assert_eq!(left, right, "get mismatch for key {key}");
        }
        Op::Iterate => {
            let lhs: HashSet<_> = map.iter().map(|(k, v)| (*k, *v)).collect();
            let rhs: HashSet<_> = shadow.iter().map(|(k, v)| (*k, *v)).collect();
            assert_eq!(lhs, rhs, "iter mismatch");
        }
        Op::CloneMap => {
            let cloned = map.clone();
            for (key, value) in map.iter() {
                assert_eq!(cloned.get(key), Some(value), "clone mismatch for key {key}");
            }
        }
    }

    debug_assert_eq!(map.len(), shadow.len(), "length divergence after apply_op");
}

