#![allow(unused_variables, unused_imports, unused)]

// mod lib;
mod lib2;
mod structs;
mod collections;

use ferris_says::say;
use std::{env, fs};
use std::fs::File;
use std::io::{stdout, BufWriter};
use std::path::{PathBuf, Path};
use hello::*;
use crate::collections::IPAddress;
use crate::structs::{Person, User};
use rand::prelude::*;

const FOO_BAR: i32 = 12345_i32;

fn main() {
    // let args: Vec<String> = std::env::args().skip(1).collect();
    // for arg in args { println!("arg: {}", arg); }

    ///// String, &str, mut
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    lib2::print_type_of(&arg);

    lib2::inspect(&arg);
    lib2::change(&mut arg);
    println!("I have many {}", arg);

    if lib2::eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    lib2::bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
    lib2::print_type_of(&material);
    println!("");


    //// Variables, constants
    if FOO_BAR < 0 {
        panic!("Just some random error!");
    }

    let borrowed_string_slice = "Hey"; // Literal string = borrowed string slice
    let a_string: String = borrowed_string_slice.to_string();

    let first_str = String::from("value");
    let second_str = first_str.clone();
    println!("{:p}", &first_str);
    println!("{:p}", &second_str);

    let (mut foo, bar) = (3.14_f32, 'c');
    let info = ("contact", "name", 123);
    foo = 3.142;
    println!("\nHello Rust! \ntuple: ({}, {}) - {} - const: {}", foo, bar, info.0, FOO_BAR);
    let arr = [0; 3];
    println!("Array {:?}\n", arr);


    //// match Result<Ok, Err>
    match check_size(-1) {
        Ok(_) => { print!("Yay!\n"); }
        Err(msg) => { print!("-1: {}\n", msg); }
    }
    match check_size(1) {
        Ok(_) => { print!("1: Yay!\n"); }
        Err(msg) => { print!("{}\n", msg); }
    }
    print!("\n");


    //// modules, strings, references
    print!("hello::nonse() called: {}\n", hello::nonse(1, 2.0));
    print!("nonse() called: {}\n", nonse(1, 2.0));
    print!("hello::find() called: {} - {}\n", hello::find("k1"), find("k2"));

    let str3 = String::from("value");
    with_reference(&str3);
    print!("hello::with_reference() called: {}\n", str3);

    let mut str4 = String::from("value");
    with_reference_mut(&mut str4);
    print!("hello::with_reference_mut() called: {}\n", str4);


    //// controls, loops
    flow_controls();
    println!("");


    //// Struct, Trait, Enum
    let ip1 = IPAddress::V4(127, 0, 0, 1);
    let mut person1 = structs::Person::new(String::from("P"), String::from("M"), 30, ip1);
    person1.update();
    person1.print();
    structs::is_user_old_enough(person1);

    let ip2 = IPAddress::V6(String::from("A0:B1:C2"));
    let mut person2 = structs::Person::new(String::from("M"), String::from("P"), 10, ip2);
    person2.fname = String::from("M");
    person2.print();
    println!("");


    //// Option, Result
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
    println!("{}", file_content);


    //// ferris_says
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap_or_default();
}

