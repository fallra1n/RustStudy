use combinations::combinations;

fn main() {
    let arr = vec![1, 2, 3, 4];
    let k = 2;

    let result = combinations(&arr, k);

    println!("Combinations: {:?}", result);
}