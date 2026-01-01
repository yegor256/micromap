// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-FileCopyrightText: Copyright (c) 2025 owtotwo
// SPDX-License-Identifier: MIT

/// Use micromap instead of HashMap in std to implement the classic data structure
/// of LRU Cache as usage sample.
/// (Ref: https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU)
use micromap::Map as MicroMap;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

mod lru_cache {
    use super::*;
    use map_in_lru::*;

    #[derive(Debug, Clone)]
    pub struct LRUCache<K, V, const N: usize, M: Map<K, V> = MicroMap<K, V, N>> {
        capacity: usize,
        map: M,
        order: VecDeque<K>,
        _marker: std::marker::PhantomData<V>,
    }

    // LRUCache core API
    impl<K: Hash + Eq + Clone, V, const N: usize, M: Map<K, V> + WithCapacity> LRUCache<K, V, N, M> {
        pub fn new() -> Self {
            LRUCache {
                capacity: const { N },
                map: M::with_capacity(N),
                order: VecDeque::new(),
                _marker: std::marker::PhantomData,
            }
        }

        pub fn get(&mut self, key: &K) -> Option<&V> {
            if self.map.contains_key(key) {
                self.order.retain(|k| k != key);
                self.order.push_back(key.clone());
                self.map.get(key)
            } else {
                None
            }
        }

        pub fn put(&mut self, key: K, value: V) {
            if self.map.contains_key(&key) {
                self.order.retain(|k| k != &key);
            } else if self.map.len() == self.capacity {
                if let Some(old_key) = self.order.pop_front() {
                    self.map.remove(&old_key);
                }
            }
            self.order.push_back(key.clone());
            self.map.insert(key, value);
        }
    }
}

mod map_in_lru {
    use super::*;

    // Map in LRUCache has two trait requirements

    pub trait Map<K, V> {
        fn contains_key(&self, key: &K) -> bool;
        fn get(&self, key: &K) -> Option<&V>;
        fn insert(&mut self, key: K, value: V);
        fn remove(&mut self, key: &K) -> Option<V>;
        fn len(&self) -> usize;
    }

    pub trait WithCapacity {
        fn with_capacity(capacity: usize) -> Self;
    }

    // Implementations for HashMap

    impl<K: Eq + Hash, V> Map<K, V> for HashMap<K, V> {
        fn contains_key(&self, key: &K) -> bool {
            HashMap::contains_key(self, key)
        }

        fn get(&self, key: &K) -> Option<&V> {
            HashMap::get(self, key)
        }

        fn insert(&mut self, key: K, value: V) {
            HashMap::insert(self, key, value);
        }

        fn remove(&mut self, key: &K) -> Option<V> {
            HashMap::remove(self, key)
        }

        fn len(&self) -> usize {
            HashMap::len(self)
        }
    }

    impl<K, V> WithCapacity for HashMap<K, V> {
        fn with_capacity(capacity: usize) -> Self {
            HashMap::with_capacity(capacity)
        }
    }

    // Implementations for MicroMap

    impl<K: PartialEq, V, const N: usize> Map<K, V> for MicroMap<K, V, N> {
        fn contains_key(&self, key: &K) -> bool {
            MicroMap::contains_key(self, key)
        }

        fn get(&self, key: &K) -> Option<&V> {
            MicroMap::get(self, key)
        }

        fn insert(&mut self, key: K, value: V) {
            MicroMap::insert(self, key, value);
        }

        fn remove(&mut self, key: &K) -> Option<V> {
            MicroMap::remove(self, key)
        }

        fn len(&self) -> usize {
            MicroMap::len(self)
        }
    }

    impl<K: PartialEq, V, const N: usize> WithCapacity for MicroMap<K, V, N> {
        fn with_capacity(capacity: usize) -> Self {
            assert_eq!(capacity, N);
            MicroMap::<K, V, N>::new()
        }
    }
}

fn main() {
    use lru_cache::LRUCache;
    const MAX: usize = 2; // LRU Cache Capacity
    {
        // micromap::Map
        let mut cache: LRUCache<_, _, MAX> = LRUCache::new(); //use MicroMap by default
        cache.put(1, 1);
        cache.put(2, 2);
        println!("{:?}", cache.get(&1)); // Some(&1)
        cache.put(3, 3);
        println!("{:?}", cache.get(&2)); // None (removed)
        cache.put(4, 4);
        println!("{:?}", cache.get(&1)); // None (removed)
        println!("{:?}", cache.get(&3)); // Some(&3)
        println!("{:?}", cache.get(&4)); // Some(&4)
    }
    {
        // std::collection::HashMap
        let mut cache: LRUCache<_, _, MAX, HashMap<_, _>> = LRUCache::new();
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(&1), Some(&1)); // Some(&1)
        cache.put(3, 3);
        assert_eq!(cache.get(&2), None); // None (removed)
        cache.put(4, 4);
        assert_eq!(cache.get(&1), None); // None (removed)
        assert_eq!(cache.get(&3), Some(&3)); // Some(&3)
        assert_eq!(cache.get(&4), Some(&4)); // Some(&4)
    }
}
