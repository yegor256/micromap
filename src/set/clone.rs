// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Set;

impl<T: Clone, const N: usize> Clone for Set<T, N> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}
