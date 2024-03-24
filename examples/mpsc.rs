use mpsc::channel;

fn main() {
    let (sender, mut receiver) = channel::<i32>();
    for k in 0..10 {
        for i in k..k + 10 {
            sender.send(i % 10).unwrap();
        }
        for i in k..k + 10 {
            println!("{} == {}", receiver.recv().unwrap(), i % 10);
        }
    }
}
