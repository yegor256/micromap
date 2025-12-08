// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![allow(clippy::new_without_default)]

pub mod apply;
pub mod input;

pub const MAX_CAPACITY: usize = 16;

pub use apply::apply_op;
pub use input::{FuzzInput, Op, MAX_OPS};

