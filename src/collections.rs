#![allow(unused_variables, unused_imports)]

use std::vec;
use std::collections::HashMap;

//// Vector

pub fn create_vector() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(1);
    vector.push(1);

    println!("{}", vector[0]);
    let x = vector.pop();

    let mut vec2 = vec!['a', 'b'];
}

//// HashMap
pub fn find(key: &str) -> String {
    let mut books = HashMap::new();
    books.insert("k1", "v1");
    books.insert("k2", "v2");

    println!("\nHashMap loop => ");
    for (key, value) in &books {
        print!("({}: {}) ", key, value);
    }
    println!("{:?}", books);

    return books.get(key).unwrap().to_string();
}

/////////////////////////////////
// Enum

#[derive(Debug)]
pub enum IPAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IPAddress {
    pub fn to_string(&self) -> String {
        return match self {
            Self::V4(a, b, c, d) => format!("IPv4 [{}.{}.{}.{}]", a, b, c, d),
            Self::V6(ip) => format!("IPv6 [{}]", ip),
        }
    }
}
