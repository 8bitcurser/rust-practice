use std::thread;
use std::sync::{Arc, Mutex, mpsc};


type Job = Box<FnBox + Send + 'static>;
type RecvJob = Arc<Mutex<mpsc::Receiver<Message>>>;

trait FnBox {
    // takes ownership of self so it can move the
    // value out of the Box
    fn call_box(self: Box<Self>);
}

pub struct ThreadPool {
    // The type of JoinHandle is () [unit type] given that we return nothing
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

struct Worker {
    id: usize,
    // wrapping in an option allows us to take the value
    // from some and leave a None at the place, this way
    // we can take ownership of the thread.
    thread: Option<thread::JoinHandle<()>>
}

enum Message {
    NewJob(Job),
    Terminate
}

// we implement the trait for any type that
// implements the FnOnce trait with the unity type
// meaning it returns nothing
impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>){
        (*self)()
    }
}

impl Worker {
    fn new(id: usize, receiver: RecvJob) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // lock to acquire mutex
                // unwrap to panic on any erros
                // recv to obtain the job from the channel
                // unwrap to panic but obtain the adequate Job
                let msg = receiver.lock().unwrap().recv().unwrap();
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; exeucting.", id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
                
            }
        });
        Worker {
            id,
            thread: Some(thread)
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        /// The size is the number of threads in the pool
        ///
        ///  # Panics
        ///
        ///  The `new` function will panic if the size is 0
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // define a vector with a preallocated size, its more performant if
        // we know before hand how many elements are going inside the vec
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender
        }
    }
    pub fn execute<F>(&self, f: F) 
    where
        // the generic type must have fnonce to accept the clousure,
        // send to be able to send the clousure to another thread
        // and the lifetime static because we dont know how long it will take 
        // to run, we do FnOnce() instead of FnOnce cause this clousure doesnt
        // take parameters nor returns anything.
        F: FnOnce() + Send + 'static {
            let job = Box::new(f);
            self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending termination message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting all workers down!");
        for worker in &mut self.workers {
            // as we made thread an Option we now can take the value from it
            // if any and call the join method
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
