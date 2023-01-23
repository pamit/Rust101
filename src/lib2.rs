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

pub fn call(arg: &mut String) {
    print_type_of(&arg);

    inspect(&arg);
    change(arg);
    println!("I have many {}", arg);

    if eat(arg.to_string()) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
    print_type_of(&material);
    println!("");
}
