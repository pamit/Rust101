#![allow(unused_variables, unused_imports, unused)]

// mod lib;
mod lib2;
mod structs;
mod collections;
mod closures;
mod option;
mod variables;
mod threads;

use ferris_says::say;
use std::{env, fs};
use std::fs::File;
use std::io::{stdout, BufWriter};
use std::path::{PathBuf, Path};
use hello::*;
use crate::collections::IPAddress;
use crate::lib2::print_type_of;
use crate::structs::{Person, User};
use rand::prelude::*;


fn main() {
    // let args: Vec<String> = std::env::args().skip(1).collect();
    // for arg in args { println!("arg: {}", arg); }

    ///// String, &str, mut
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        // println!("Please supply an argument to this program.");
        // std::process::exit(-1);
        return String::from("apple");
    });

    lib2::call(&mut arg);


    //// Variables, constants
    variables::call();


    //// modules, strings, references
    print!("hello::nonse() called: {}\n", hello::nonse(1, 2.0));
    print!("nonse() called: {}\n", nonse(1, 2.0));
    print!("hello::find() called: {} - {}\n", collections::find("k1"), collections::find("k2"));

    let str3 = String::from("value");
    with_reference(&str3);
    print!("hello::with_reference() called: {}\n", str3);

    let mut str4 = String::from("value");
    with_reference_mut(&mut str4);
    print!("hello::with_reference_mut() called: {}\n", str4);


    //// controls, loops
    flow_controls();


    //// Struct, Trait, Enum
    structs::call();


    //// Option, Result
    option::call();


    //// Closures
    closures::call();


    //// Threads
    threads::call();
    threads::call_move();
    threads::call_message_transfer();


    //// ferris_says
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap_or_default();
}
