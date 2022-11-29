//! A toy implementatino of a thread pool that maintains a fixed number of
//! worker threads and passes incoming task to idle workers
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

/// Type declaration for "a closure that can be passed across threads"
type Job = Box<dyn FnOnce() + Send + 'static>;

/// representing a collection of workers
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
            sender: Some(sender),
        }
    }

    /// Send the input closure to one of the worker for execution
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        if let Some(_sender) = &self.sender {
            _sender.send(Box::new(f)).unwrap();
        }
    }
}

/// Gracefully shutdown the threadpool by letting each worker finish serving
/// its current request before worker thread joins the main thread
impl Drop for ThreadPool {
    /// First drop the sender. Dropping the (only) sender will close the
    /// message channel, and worker calling receiver will receive errors.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(_handle) = worker.handle.take() {
                _handle.join().unwrap();
            }
            println!("Worker {} gracefully exited", worker.id);
        }
    }
}

/// Each worker holds an atomic reference counter on a mutex of a mpsc:Receiver
/// so that multiple worker can receive closure from the thread pool to execute
struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let job = receiver
                    .lock()  // Result<MutexGuard<...>>
                    .unwrap()  // MutexGuard<Receiver<Job>>
                    .recv();  // Result<Job, Err>

                match job {
                    Ok(_job) => {
                        println!("worker {id} received a new job");
                        _job();
                    },
                    Err(_) => {
                        println!("Channel closed. Worker {id} exiting");
                        return ();
                    },
                }
            }
        });
        return Worker{ id, handle: Some(handle) };
    }
}
