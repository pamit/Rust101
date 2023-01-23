const FOO_BAR: i32 = 12345_i32;

pub fn call() {
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
}
