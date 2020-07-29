use std::thread;
use std::sync::mpsc;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // we are using rx as an iterator, this is like saying
    // he iterate over the values you receive and dispatch it as they come 
    // while the channel is still open
    for received in rx {
        println!("Got: {}", received);
    }
}
