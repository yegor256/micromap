// Copyright (c) 2023-2024 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::Set;
use core::fmt::Formatter;
use core::marker::PhantomData;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<T: PartialEq + Serialize, const N: usize> Serialize for Set<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for k in self.iter() {
            seq.serialize_element(k)?;
        }
        seq.end()
    }
}

struct Vi<T, const N: usize>(PhantomData<T>);

impl<'de, T: PartialEq + Deserialize<'de>, const N: usize> Visitor<'de> for Vi<T, N> {
    type Value = Set<T, N>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("a Set")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut m: Self::Value = Set::new();
        while let Some(key) = seq.next_element()? {
            m.insert(key);
        }
        Ok(m)
    }
}

impl<'de, T: PartialEq + Deserialize<'de>, const N: usize> Deserialize<'de> for Set<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(Vi(PhantomData))
    }
}

#[cfg(test)]
use bincode::{deserialize, serialize};

#[test]
fn serialize_and_deserialize() {
    let mut before: Set<u8, 8> = Set::new();
    before.insert(1);
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Set<u8, 8> = deserialize(&bytes).unwrap();
    assert_eq!(1, after.into_iter().next().unwrap());
}

#[test]
fn empty_set_serde() {
    let before: Set<u8, 8> = Set::new();
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Set<u8, 8> = deserialize(&bytes).unwrap();
    assert!(after.is_empty());
}
