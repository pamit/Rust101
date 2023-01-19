#![allow(unused_variables, unused_imports)]

pub fn inspect(s: &str) {
    if s.ends_with("s") {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

pub fn bedazzle(s: &mut String) {
    (*s) = "sparkly".to_string(); // dereference s first
    // s.push_str("s");           // dereferenced by .
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
