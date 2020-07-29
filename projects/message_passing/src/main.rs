use std::sync::mpsc;
use std::thread;

fn main() {
    // tx transmitter
    // rx receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        // send message through channel
        tx.send(val).unwrap();
        // this wouldnt work as the val variable was moved after being sent
        // println!("val is {}", val);
        // borrowing rules protect this behaviour
    });
    // send returns a Result<T, E>
    // propper error handling could be introuduced
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
