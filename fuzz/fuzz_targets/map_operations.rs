// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;
use micromap::Map;

// Fuzz target for Map operations
fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let mut map: Map<u8, u8, 16> = Map::new();
    let mut i = 0;

    while i < data.len() {
        let op = data[i] % 8; // 8 different operations
        i += 1;

        match op {
            0 => {
                // Insert operation
                if i + 1 < data.len() {
                    let key = data[i];
                    let value = data[i + 1];
                    i += 2;
                    
                    // Use checked_insert to avoid panics on full map
                    let _ = map.checked_insert(key, value);
                }
            }
            1 => {
                // Get operation
                if i < data.len() {
                    let key = data[i];
                    i += 1;
                    let _ = map.get(&key);
                }
            }
            2 => {
                // Remove operation
                if i < data.len() {
                    let key = data[i];
                    i += 1;
                    let _ = map.remove(&key);
                }
            }
            3 => {
                // Contains key operation
                if i < data.len() {
                    let key = data[i];
                    i += 1;
                    let _ = map.contains_key(&key);
                }
            }
            4 => {
                // Clear operation
                map.clear();
            }
            5 => {
                // Len operation
                let _ = map.len();
            }
            6 => {
                // Is empty operation
                let _ = map.is_empty();
            }
            7 => {
                // Retain operation - keep only even keys
                map.retain(|k, _| *k % 2 == 0);
            }
            _ => unreachable!(),
        }
    }

    // Test iteration to ensure no corruption
    for (k, v) in &map {
        // Basic invariant: key and value should be valid u8 values
        let _ = *k;
        let _ = *v;
    }
});
