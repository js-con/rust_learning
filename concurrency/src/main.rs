use std::{thread, time::Duration, sync::mpsc};

fn main() {
    let v = vec![1, 2, 3];
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = "hi".to_string();
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
