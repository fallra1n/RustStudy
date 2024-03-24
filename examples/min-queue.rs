use min_queue::MinQueue;

fn main() {
    let mut queue = MinQueue::new();
    queue.push(2);
    queue.push(1);
    println!("front : {:?}", queue.front());
    println!("min : {:?}", queue.min());
}