#![no_main]

use libfuzzer_sys::fuzz_target;
use micromap::Set;

// Fuzz target for Set operations
fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let mut set: Set<u8, 16> = Set::new();
    let mut i = 0;

    while i < data.len() {
        let op = data[i] % 8; // 8 different operations
        i += 1;

        match op {
            0 => {
                // Insert operation
                if i < data.len() {
                    let value = data[i];
                    i += 1;
                    
                    // Use checked_insert to avoid panics on full set
                    let _ = set.checked_insert(value);
                }
            }
            1 => {
                // Contains operation
                if i < data.len() {
                    let value = data[i];
                    i += 1;
                    let _ = set.contains(&value);
                }
            }
            2 => {
                // Remove operation
                if i < data.len() {
                    let value = data[i];
                    i += 1;
                    let _ = set.remove(&value);
                }
            }
            3 => {
                // Get operation
                if i < data.len() {
                    let value = data[i];
                    i += 1;
                    let _ = set.get(&value);
                }
            }
            4 => {
                // Clear operation
                set.clear();
            }
            5 => {
                // Len operation
                let _ = set.len();
            }
            6 => {
                // Is empty operation
                let _ = set.is_empty();
            }
            7 => {
                // Retain operation - keep only even values
                set.retain(|v| *v % 2 == 0);
            }
            _ => unreachable!(),
        }
    }

    // Test iteration to ensure no corruption
    for value in &set {
        // Basic invariant: value should be a valid u8
        let _ = *value;
    }

    // Test set-specific operations if we have enough data
    if data.len() > 10 {
        let mut other_set: Set<u8, 8> = Set::new();
        for &val in data.iter().take(8) {
            let _ = other_set.checked_insert(val);
        }

        // Test set operations
        let _ = set.is_disjoint(&other_set);
        let _ = set.is_subset(&other_set);
        let _ = set.is_superset(&other_set);
    }
});