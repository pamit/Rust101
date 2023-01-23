use rand::prelude::*;
use std::{env, fs};

pub fn call() {
    let mut x: Option<i32> = None;
    let random = rand::thread_rng().gen::<u8>();
    if random <= 10 {
        x = Some(10);
    }
    // if x.is_some() { println!("Matched: Some({})", x); } else { println!("Matched: None"); }
    match x {
        None => { println!("Matched: None"); }
        Some(y) => { println!("Matched: Some({})", y); }
    }
    println!("");

    let dir = String::from("/Users/payammousavi/Documents/Personal/Projects/Rust/hello-rust");
    // let mut dir_result = env::current_dir();
    // let mut path: PathBuf = dir_result.unwrap();
    // let file = Path::new("Cargo.toml");
    // path.push(file);
    // let result = File::open(path);
    // if result.is_err() {
    //     panic!("Cannot open file");
    // }
    // let file = result.unwrap();
    let file_content = fs::read_to_string(dir + "/Cargo.toml").expect("Failed to read the file");
    println!("--- File ----");
    println!("{}\n", file_content);
}
