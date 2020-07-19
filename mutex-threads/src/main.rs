use std::sync::{Mutex, Arc};
use std::thread;


fn main() {
    // we wrap so the counter can be called by all if not the first child 
    // thread will own the counter and it wont be usable by the rest
    // but Rc is not safe for more than one thread as it can control the counts
    // properly, we need Arc<T> [Atomic reference count]
    // this works similar to Rc but safe for concurrency, the disadvantage is
    // it comes with overhead.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
