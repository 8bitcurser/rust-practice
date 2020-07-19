use std::sync::Mutex;

fn main() {
    // we declare a mutex with the value 5
    let m = Mutex::new(5);
    // generate a scope
    {
        // lock the value at the mutex, and obtain the inner ref for it
        // make the ref mutable
        // lock blocks the current thread so it doesn't do anything else til
        // it obtains the lock.
        // this will panic if someone else is having the lock
        // lock returns a MutexGuard instance which is an SP
        let mut num = m.lock().unwrap();
        // change value inside ref
        *num = 6;

        // when the scope is droped the lock is free
    }
    println!("m = {:?}", m);
}
