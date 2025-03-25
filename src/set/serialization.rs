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
        for k in self {
            seq.serialize_element(k)?;
        }
        seq.end()
    }
}

struct Vi<T, const N: usize>(PhantomData<T>);

impl<'de, T: PartialEq + Deserialize<'de>, const N: usize> Visitor<'de> for Vi<T, N> {
    type Value = Set<T, N>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
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
    use bincode::serde::{decode_from_slice, encode_into_slice};

    #[test]
    fn serialize_and_deserialize() {
        let config = bincode::config::legacy();
        let mut before: Set<u8, 8> = Set::new();
        before.insert(1);
        let mut bytes: [u8; 1024] = [0; 1024];
        let len = encode_into_slice(&before, &mut bytes, config).unwrap();
        let bytes = &bytes[..len];
        let (after, read_len): (Set<u8, 8>, usize) = decode_from_slice(&bytes, config).unwrap();
        assert_eq!(1, after.into_iter().next().unwrap());
        assert_eq!(bytes.len(), read_len);
    }

    #[test]
    fn empty_set_serde() {
        let config = bincode::config::legacy();
        let before: Set<u8, 8> = Set::new();
        let mut bytes: [u8; 1024] = [0; 1024];
        let len = encode_into_slice(&before, &mut bytes, config).unwrap();
        let bytes = &bytes[..len];
        let (after, read_len): (Set<u8, 8>, usize) = decode_from_slice(&bytes, config).unwrap();
        assert!(after.is_empty());
        assert_eq!(bytes.len(), read_len);
    }
}
