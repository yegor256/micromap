// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;
use micromap::Map;

// Fuzz target for edge cases and complex Map scenarios
fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    // Test different capacities based on input
    let capacity_choice = data[0] % 4;
    match capacity_choice {
        0 => test_map_capacity::<1>(data),
        1 => test_map_capacity::<4>(data),
        2 => test_map_capacity::<8>(data),
        3 => test_map_capacity::<32>(data),
        _ => unreachable!(),
    }
});

fn test_map_capacity<const N: usize>(data: &[u8]) {
    let mut map: Map<u16, u16, N> = Map::new();
    let mut i = 1;

    while i + 3 < data.len() {
        let op = data[i] % 10;
        let key = u16::from_le_bytes([data[i + 1], data[i + 2]]);
        let value = u16::from_le_bytes([data[i + 2], data[i + 3]]);
        i += 4;

        match op {
            0..=3 => {
                // Insert operations (higher probability)
                match map.checked_insert(key, value) {
                    Some(_) => {
                        // Verify the insert worked
                        assert_eq!(map.get(&key), Some(&value));
                    }
                    None => {
                        // Map is full, which is expected behavior
                        assert_eq!(map.len(), N);
                    }
                }
            }
            4 => {
                // Get operation
                if let Some(retrieved_value) = map.get(&key) {
                    // Value must be consistent
                    assert!(map.contains_key(&key));
                    // Get the same value again
                    assert_eq!(map.get(&key), Some(retrieved_value));
                }
            }
            5 => {
                // Remove operation
                let old_len = map.len();
                let old_contains = map.contains_key(&key);
                let removed = map.remove(&key);
                
                if removed.is_some() {
                    assert!(old_contains);
                    assert_eq!(map.len(), old_len - 1);
                    assert!(!map.contains_key(&key));
                } else {
                    assert!(!old_contains);
                    assert_eq!(map.len(), old_len);
                }
            }
            6 => {
                // get_mut operation
                if let Some(value_ref) = map.get_mut(&key) {
                    let new_value = value.wrapping_add(1);
                    *value_ref = new_value;
                    assert_eq!(map.get(&key), Some(&new_value));
                }
            }
            7 => {
                // Entry API operations
                let len_before = map.len();
                match map.entry(key) {
                    micromap::map::Entry::Occupied(mut entry) => {
                        entry.insert(value);
                        assert_eq!(map.get(&key), Some(&value));
                    }
                    micromap::map::Entry::Vacant(entry) => {
                        if len_before < N {
                            entry.insert(value);
                            assert_eq!(map.get(&key), Some(&value));
                        }
                    }
                }
            }
            8 => {
                // Retain operation with complex predicate
                let old_len = map.len();
                map.retain(|k, v| (*k as u32 + *v as u32) % 3 != 0);
                assert!(map.len() <= old_len);
                
                // Verify all remaining elements satisfy the predicate
                for (k, v) in &map {
                    assert!((*k as u32 + *v as u32) % 3 != 0);
                }
            }
            9 => {
                // Stress test iteration consistency
                let collected_keys: Vec<u16> = map.keys().copied().collect();
                let collected_values: Vec<u16> = map.values().copied().collect();
                let collected_pairs: Vec<(u16, u16)> = map.iter().map(|(k, v)| (*k, *v)).collect();
                
                assert_eq!(collected_keys.len(), map.len());
                assert_eq!(collected_values.len(), map.len());
                assert_eq!(collected_pairs.len(), map.len());
                
                // Verify each collected key exists in the map
                for key in collected_keys {
                    assert!(map.contains_key(&key));
                }
            }
            _ => unreachable!(),
        }

        // Invariants that should always hold
        assert!(map.len() <= N);
        assert_eq!(map.is_empty(), map.len() == 0);
        assert_eq!(map.capacity(), N);
        
        // Count actual entries vs reported length
        let actual_count = map.iter().count();
        assert_eq!(actual_count, map.len());
    }
}