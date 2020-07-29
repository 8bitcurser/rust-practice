use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();
    // we produce another sender
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("1hi"),
            String::from("1from"),
            String::from("1the"),
            String::from("1other"),
            String::from("1thread")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
