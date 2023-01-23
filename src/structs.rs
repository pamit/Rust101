#![allow(unused_variables, unused_imports)]

// #[path = "collectionss.rs"] mod collectionss;
// use collectionss::IPAddress;

use crate::collections::IPAddress;

pub struct Person {
    pub fname: String,
    pub lname: String,
    pub age: u16,
    pub ip: IPAddress
}

impl Person {
    pub fn new(fname: String, lname: String, age: u16, ip: IPAddress) -> Self {
        // Person { fname, lname, age }
        Self { fname, lname, age, ip }
    }

    pub fn print(&self) {
        println!("This person is {} {}, {} yo - can work? {} - {}",
            self.fname, self.lname, self.age, Person::is_old_enough(&self), self.ip.to_string());
        Person::some_default_method();
    }
}

/////////////////////////////////////
// Trait
pub trait User {
    fn name(&self) -> String;
    fn is_old_enough(&self) -> bool;
    fn some_default_method() {
        println!("Hey default method!");
    }
    fn update(self: &mut Self);
}

impl User for Person {
    fn is_old_enough(&self) -> bool {
        return self.age > 18;
    }

    fn name(&self) -> String {
        return format!("{} {}", self.fname, self.lname);
    }

    fn update(self: &mut Self) {
        self.age = 50;
    }
}

pub fn is_user_old_enough<T: User>(o: T) {
    println!("User [{}] is old enough: {}", o.name(), o.is_old_enough());
}

pub fn call() {
    let ip1 = IPAddress::V4(127, 0, 0, 1);
    let mut person1 = Person::new(String::from("P"), String::from("M"), 30, ip1);
    person1.update();
    person1.print();
    is_user_old_enough(person1);

    let ip2 = IPAddress::V6(String::from("A0:B1:C2"));
    let mut person2 = Person::new(String::from("M"), String::from("P"), 10, ip2);
    person2.fname = String::from("M");
    person2.print();
    println!("");
}
