pub fn call() {
    let add = |x, y| { x + y };
    println!("Closure::add => {}", add(2, 3));


    let str5 = "foo".to_string();
    let foo = || {
        println!("Closure::foo => {}", str5);
    };
    call_closure(foo);
    foo();
    println!("str5 => {}", str5);


    let mut v = vec![2, 4, 6];
    let result = v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);
    println!("Result of map.filter.fold: {}\n", result);

    let mut list = vec![1,2,3];
    let mut borrows_mutably_closure = || { list.push(4) };
    // let mut borrows_mutably_closure = || list.push(4);
    borrows_mutably_closure();
    println!("borrows_mutably_closure: {:?}", list);
}

fn call_closure<F: Fn()>(func: F) {
    print!("calling closure... ");
    func();
}
