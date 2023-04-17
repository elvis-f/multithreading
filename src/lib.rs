use std::{
    thread,
    sync::{
        mpsc, Arc, Mutex
    },
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers,
            sender: Some(sender)
        }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), String>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        let sender = self.sender.as_ref()
            .ok_or("Could not reference sender!")?;

        sender.send(job)
            .map_err(|err| format!("Could not send job to worker: {:?}", err))?;

        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                if let Err(err) = thread.join() {
                    eprintln!("Error shutting down worker: {:?}", err)
                }
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {

            let message = match receiver.lock() {
                Ok(r) => match r.recv() {
                    Ok(m) => Ok(m),
                    Err(_) => {
                        println!("Failed to receive message from channel; shutting down.");
                        Err(())
                    }
                },
                Err(_) => {
                    println!("Failed to acquire lock on receiver; shutting down.");
                    Err(())
                }
            };
        
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
        
                    job();
                }
                Err(_) => {
                    println!("Worker {id} received an error.");
                    break;
                }
            }
        });

        Worker { 
            id,
            thread: Some(thread),
        }
    }
}