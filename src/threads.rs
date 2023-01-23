use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn call() {
    let thread_handler = thread::spawn(|| {
        for i in 1..=5 {
            println!("thread #{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // thread_handler.join().unwrap();

    for i in 1..=3 {
        println!("thread #{} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread_handler.join().unwrap();
}

pub fn call_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("[Threads with move] Here's a vector: {:?}\n", v);
    });

    handle.join().unwrap();
}

pub fn call_message_transfer() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // let val = String::from("value sent to channel");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    for received in rx {
        println!("[Channel] message received: {}", received);
    }
}
