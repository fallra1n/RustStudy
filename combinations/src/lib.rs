#![forbid(unsafe_code)]

fn generate_combinations(
    arr: &[i32],
    start: usize,
    idx: usize,
    k: usize,
    combination: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if idx == k {
        result.push(combination.clone());
        return;
    }
    for i in start..arr.len() {
        combination[idx] = arr[i];
        generate_combinations(arr, i + 1, idx + 1, k, combination, result);
    }
}

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut combination = vec![0; k];

    generate_combinations(arr, 0, 0, k, &mut combination, &mut result);
    result
}
