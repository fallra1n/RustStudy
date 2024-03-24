use comm;
use std::env::current_dir;
use std::{env, path::PathBuf};

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!();
        println!("[WARN]: No command-line arguments provided, using default values");

        let current_dir = current_dir().unwrap();

        let mut path1 = PathBuf::from(current_dir.clone());
        path1.push("combinations.rs");

        let mut path2 = PathBuf::from(current_dir.clone());
        path2.push("conway.rs");

        args.push(path1.to_str().unwrap().to_string());
        args.push(path2.to_str().unwrap().to_string());
    }

    println!();
    println!("[COMM]");

    comm::comm(&args[1], &args[2]);
}
