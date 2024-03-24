#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut res = Vec::new();
    let strs_len = strs.len();

    let mut strs_iters = Vec::new();
    for str in &strs {
        strs_iters.push(str.chars());
    }

    let mut min_len = usize::MAX;
    for iter in &strs_iters {
        if iter.clone().count() < min_len {
            min_len = iter.clone().count();
        }
    }

    'outer: for _ in 0..min_len {
        let cur_char = strs_iters[0].next().unwrap();
        for iter in strs_iters.iter_mut().take(strs_len).skip(1) {
            if cur_char != iter.next().unwrap() {
                break 'outer;
            }
        }

        res.push(cur_char);
    }

    res.iter().collect()
}
