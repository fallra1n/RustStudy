#![forbid(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    capacity: usize,
    cache: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        LRUCache {
            capacity,
            cache: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        for i in 0..self.order.len() {
            if self.order[i] == key.clone() {
                self.order.remove(i);
                break;
            }
        }

        let res = self.cache.get(key);

        if res.is_some() {
            self.order.push_back(key.clone());
        }

        res
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut res: Option<V> = None;
        if self.cache.contains_key(&key) {
            res = self.cache.remove(&key);
            self.cache.insert(key.clone(), value);

            for i in 0..self.order.len() {
                if self.order[i] == key {
                    self.order.remove(i);
                    break;
                }
            }
            self.order.push_back(key);
        } else {
            if self.capacity == self.order.len() {
                self.cache.remove(&self.order[0]);
                self.order.remove(0);
            }
            self.cache.insert(key.clone(), value);
            self.order.push_back(key.clone());
        }

        res
    }
}
