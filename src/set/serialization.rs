// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

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
mod tests {

    use crate::Set;
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
}
