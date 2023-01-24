#![allow(unused_variables, unused_imports)]

use rand::prelude::*;

pub fn nonse(a: i32, b: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return (a as f32) * b * (rng.gen::<f32>());
}

pub fn check_size(size: i32) -> Result<(), String> {
    if size < 0 {
        return Err("Nope!".to_string());
    } else if size == 0 {
        return Err("Close!".to_string());
    }
    return Ok(());
}

pub fn with_reference(str: &String) {
    println!("{}", str);
}

pub fn with_reference_mut(str: &mut String) {
    str.insert_str(0, "[string] ");
}

pub fn flow_controls() {
    println!("\n\nflow_controls:");

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


    let msg: &str; // borrowed string slice
    msg = if true {
        "1"
    } else {
        "0"
    };
    println!("{}", msg);

    // loops
    'first_loop: loop {
        println!("first_loop");
        loop {
            println!("second_loop");
            break 'first_loop;
        }
    }

    let mut count = 1;
    while count <= 3 {
        print!("{} ", count);
        count += 1;
    }
    println!("");

    for num in [1,2,3].iter() {
        print!("{}, ", num);
    }
    println!("");

    let array = [("k1", 1), ("k2", 2)];
    for (key, value) in array.iter() {
        print!("{} => {} | ", key, value);
    }
    println!("");

    for num in 0..=3 {
        print!("{} ", num);
    }
    println!("\n");

}