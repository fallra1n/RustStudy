use lru_cache::LRUCache;

fn main() {
    let mut cache = LRUCache::new(2);
    cache.insert(1, 1);
    cache.insert(2, 2);
    println!("{:?}", cache.get(&1));
    cache.insert(3, 3);
    println!("{:?}", cache.get(&2));
}
