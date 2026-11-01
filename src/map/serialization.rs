// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::Map;
use core::fmt::Formatter;
use core::marker::PhantomData;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<K: Serialize, V: Serialize, const N: usize> Serialize for Map<K, V, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (a, v) in self {
            map.serialize_entry(a, v)?;
        }
        map.end()
    }
}

struct Vi<K, V, const N: usize>(PhantomData<K>, PhantomData<V>);

impl<'de, K: PartialEq + Deserialize<'de>, V: Deserialize<'de>, const N: usize> Visitor<'de>
    for Vi<K, V, N>
{
    type Value = Map<K, V, N>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("a Map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut m: Self::Value = Map::new();
        while let Some((key, value)) = access.next_entry()? {
            m.insert(key, value);
        }
        Ok(m)
    }
}

impl<'de, K: PartialEq + Deserialize<'de>, V: Deserialize<'de>, const N: usize> Deserialize<'de>
    for Map<K, V, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vi(PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use bincode::serde::{decode_from_slice, encode_into_slice};

    #[test]
    fn serialize_and_deserialize() {
        let config = bincode::config::legacy();
        let mut before: Map<u8, u8, 8> = Map::new();
        before.insert(1, 42);
        let mut bytes: [u8; 1024] = [0; 1024];
        let len = encode_into_slice(&before, &mut bytes, config).unwrap();
        let bytes = &bytes[..len];
        let (after, read_len): (Map<u8, u8, 8>, usize) = decode_from_slice(&bytes, config).unwrap();
        assert_eq!(42, after.into_iter().next().unwrap().1);
        assert_eq!(bytes.len(), read_len);
    }

    #[test]
    fn empty_map_serde() {
        let config = bincode::config::legacy();
        let before: Map<u8, u8, 8> = Map::new();
        let mut bytes: [u8; 1024] = [0; 1024];
        let len = encode_into_slice(&before, &mut bytes, config).unwrap();
        let bytes = &bytes[..len];
        let (after, read_len): (Map<u8, u8, 8>, usize) = decode_from_slice(&bytes, config).unwrap();
        assert!(after.is_empty());
        assert_eq!(bytes.len(), read_len);
    }

    #[test]
    fn serialize_non_empty_map() {
        let config = bincode::config::legacy();
        let mut map: Map<u8, u8, 8> = Map::new();
        map.insert(10, 20);
        map.insert(30, 40);
        let mut bytes: [u8; 1024] = [0; 1024];
        let len = encode_into_slice(&map, &mut bytes, config).unwrap();
        let bytes = &bytes[..len];
        let (deserialized_map, read_len): (Map<u8, u8, 8>, usize) =
            decode_from_slice(&bytes, config).unwrap();
        assert_eq!(map.len(), deserialized_map.len());
        assert_eq!(bytes.len(), read_len);
        for (key, value) in map {
            assert_eq!(deserialized_map.get(&key), Some(&value));
        }
    }

    #[test]
    fn deserialize_invalid_data() {
        let config = bincode::config::legacy();
        let invalid_bytes: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
        let result: Result<(Map<u8, u8, 8>, usize), _> = decode_from_slice(&invalid_bytes, config);
        assert!(result.is_err());
    }
}
