use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let tx = mpsc::Sender::clone(&tx);
        let id = i.to_string();

        let handle = thread::spawn(move || {
            tx.send(id).unwrap();
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for _ in 0..10 {
        let received = rx.recv().unwrap();
        println!("Thread: {}", received);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}