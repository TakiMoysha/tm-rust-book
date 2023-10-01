use std::thread;
use std::sync::{Arc, Mutex, mpsc};


struct PoolWorker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl PoolWorker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<WorkerTask>>>) -> PoolWorker {
        let thread = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv();
            match msg {
                Ok(task) => {
                    println!("Worker {} got a task", id);
                    task();
                }
                Err(_) => {
                    println!("Worker {} disconnected", id);
                    break;
                }
            }
        });
        // todo: implement
        // std::thread::Builder
        PoolWorker { id, thread: Some(thread), }
    }
}

type WorkerTask = Box<dyn FnOnce() + Send + 'static>;


pub struct ThreadPool {
    workers: Vec<PoolWorker>,
    sender: Option<mpsc::Sender<WorkerTask>>,
}


impl ThreadPool {
    //// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<PoolWorker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(PoolWorker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    // todo: implement
    // pub fn build(size: usiz) -> Result<ThreadPool, PoolCreationError> {}

    // pub fn spawn<F, T>(f: F) -> PoolWorker
    //     where
    //         F: FnOnce() -> T,
    //         F: Send + 'static,
    //         T: Send + 'static,
    //     {
    //     } 

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        self.sender.as_ref().unwrap().send(task).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();       
            }
        }
    }
}
