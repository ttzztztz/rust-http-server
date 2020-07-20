use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got the job, executing", id);
                job();
            }
        });

        Worker {
            id, thread,
        }
    }
}

pub struct ThreadPoll {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPoll {
    pub fn new(size: usize) -> ThreadPoll {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPoll {
            threads,
            sender,
        }
    }

    pub fn execute<Func>(&self, func: Func)
        where Func: FnOnce() + Send + 'static
    {
        let job = Box::new(func);
        self.sender.send(job).unwrap();
    }
}