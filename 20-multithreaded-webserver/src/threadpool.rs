//! A toy implementatino of a thread pool that maintains a fixed number of
//! worker threads and passes incoming task to idle workers
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

/// Type declaration for "a closure that can be passed across threads"
type Job = Box<dyn FnOnce() + Send + 'static>;

/// representing a collection of workers
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl ThreadPool {
    /// Return a new thread pool with the specified number of worker.
    /// 
    /// # Panic
    ///
    /// This function will panic if the input is 0
    pub fn new(n: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(n);
        for id in 0..n {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        println!("Spawned {n} workers");

        return ThreadPool {
            workers,
            sender,
        }
    }

    /// Send the input closure to one of the worker for execution
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        self.sender.send(Box::new(f)).unwrap();
    }
}

/// Each worker holds an atomic reference counter on a mutex of a mpsc:Receiver
/// so that multiple worker can receive closure from the thread pool to execute
struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let job = receiver
                    .lock()  // Result<MutexGuard<...>>
                    .unwrap()  // MutexGuard<Receiver<Job>>
                    .recv()  // Result<Job>
                    .unwrap();

                println!("worker {id} received a new job");
                job();
            }
        });
        return Worker{ id, handle };
    }
}
