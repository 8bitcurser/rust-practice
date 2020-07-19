use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("{} <- counted from the thread", i);
            // holds the thread execution for a given duration
            thread::sleep(Duration::from_millis(1));
        }
    });
    // when the main thread dies the child thread also dies the previous count
    // will only reach 5.
    for i in 1..5 {
        println!("{} <- Main thread count", i);
        thread::sleep(Duration::from_millis(1));
    }
}
