// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use arbitrary::{Arbitrary, Result, Unstructured};

pub const MAX_OPS: usize = 64;

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Insert { key: u8, value: u8 },
    Get { key: u8 },
    Remove { key: u8 },
    Iterate,
    CloneMap,
}

#[derive(Clone, Debug)]
pub struct FuzzInput {
    pub ops: Vec<Op>,
}

impl<'a> Arbitrary<'a> for Op {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let roll = u.int_in_range(0..=99)?;
        let op = match roll {
            0..=39 => Self::Insert {
                key: u8::arbitrary(u)?,
                value: u8::arbitrary(u)?,
            },
            40..=59 => Self::Remove {
                key: u8::arbitrary(u)?,
            },
            60..=84 => Self::Get {
                key: u8::arbitrary(u)?,
            },
            85..=94 => Self::Iterate,
            _ => Self::CloneMap,
        };
        Ok(op)
    }
}

impl<'a> Arbitrary<'a> for FuzzInput {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let len = usize::min(u.int_in_range(0..=MAX_OPS)?, MAX_OPS);
        let mut ops = Vec::with_capacity(len);
        for _ in 0..len {
            ops.push(Op::arbitrary(u)?);
        }
        Ok(Self { ops })
    }
}

