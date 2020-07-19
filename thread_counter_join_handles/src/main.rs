use std::thread;
use std::time::Duration;


fn main() {
    // returns a JoinHandle (owned value)  instances when we call the
    // join method it waits till the child thread finishes
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} <- child thread count", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // if we place the handle.join().unwrap() here, before the for loop of the
    // main thread runs it will wait for the child to finish and then run the
    // for loop of the main thread.
    for i in 1..5 {
        println!("{} <- main thread count", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
