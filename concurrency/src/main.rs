use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Vector {v:?}");
    });
    handle.join().unwrap();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(0));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(0));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(0));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    let original_counter = Arc::new(Mutex::new(0));

    let counter = Arc::clone(&original_counter);
    thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    }).join().unwrap();
    let counter = Arc::clone(&original_counter);
    thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num -= 1;
    }).join().unwrap();
    println!("Result: {}", *original_counter.lock().unwrap());
}
