#![forbid(unsafe_code)]

use std::mem;
use std::{borrow::Borrow, iter::FromIterator, ops::Index};
////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

impl<K: Ord, V> FlatMap<K, V> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        return match self.0.binary_search_by_key(&key.borrow(), |(k, _)| k) {
            Ok(id) => Some(mem::replace(&mut self.0[id], (key, value)).1),
            Err(id) => {
                if id >= self.0.len() {
                    self.0.push((key, value));
                } else {
                    let tail = self.0.split_off(id);
                    self.0.push((key, value));
                    self.0.extend(tail);
                }
                None
            }
        };
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Ord + ?Sized + ToOwned<Owned = K>,
    {
        let key_k: &K = &key.to_owned();
        match self.0.binary_search_by_key(&key_k, |(k, _)| k) {
            Ok(id) => Some(&self.0[id].1),
            Err(_) => None,
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: Ord + ?Sized + ToOwned<Owned = K>,
    {
        let key_k: &K = &key.to_owned();
        match self.0.binary_search_by_key(&key_k, |(k, _)| k) {
            Ok(id) => Some(self.0.remove(id).1),
            Err(_) => None,
        }
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Ord + ?Sized + ToOwned<Owned = K>,
    {
        let key_k: &K = &key.to_owned();
        match self.0.binary_search_by_key(&key_k, |(k, _)| k) {
            Ok(id) => Some(self.0.remove(id)),
            Err(_) => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<K: Ord, Q, V> Index<&Q> for FlatMap<K, V>
where
    Q: Ord + ?Sized + ToOwned<Owned = K>,
{
    type Output = V;
    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            (*self).insert(key, value);
        }
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for FlatMap<K, V> {
    fn from(value: Vec<(K, V)>) -> Self {
        let mut res = FlatMap::new();
        for (key, value) in value {
            res.insert(key, value);
        }
        res
    }
}

impl<K: Ord, V> From<FlatMap<K, V>> for Vec<(K, V)> {
    fn from(value: FlatMap<K, V>) -> Self {
        value.into_iter().collect()
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut res = Self::new();
        for (key, value) in iter {
            res.insert(key, value);
        }
        res
    }
}

impl<K: Ord, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
