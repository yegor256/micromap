// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! Template for a fuzz regression test.
//! Copy this file, rename it, replace the operation sequence,
//! and remove #[ignore] once the test is ready to run by default.

use micromap::Map;

#[test]
#[ignore]
fn fuzz_regression_template() {
    const MAX_CAPACITY: usize = 16;
    let mut map = Map::<u8, u8, MAX_CAPACITY>::new();

    // NOTE: Insert the restored sequence of operations from the artifact.
    map.insert(1, 42);
    map.remove(&1);
    map.insert(1, 99);

    assert_eq!(map.get(&1), Some(&99));
}


