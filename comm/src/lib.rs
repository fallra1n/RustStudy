#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::{fs::File, io::BufRead, io::BufReader};

pub fn comm(first_filename: &str, second_filename: &str) {
    let first_file = File::open(first_filename).unwrap();
    let second_file = File::open(second_filename).unwrap();

    let first_reader = BufReader::new(first_file);
    let second_reader = BufReader::new(second_file);

    let mut lines: HashSet<String> = HashSet::new();

    for line in first_reader.lines() {
        lines.insert(line.unwrap());
    }

    for line in second_reader.lines() {
        let value = line.unwrap();
        if lines.contains(&value.clone()) {
            println!("{}", value.clone());
            lines.remove(&value);
        }
    }
}
