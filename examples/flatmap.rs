use flatmap::FlatMap;

fn main() {
    let mut map = FlatMap::new();
    map.insert(1, 20);

    println!("{}", map.len());
    println!("{}", map.capacity());
    println!("{:?}", map.get(&1));
    println!("{:?}", map.as_slice());
}
