use super::err::MiruErr;
use std::thread::JoinHandle;
use std::thread::spawn;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: Sender<Job>,
}

impl ThreadPool{
    pub fn new(size: usize) -> Result<ThreadPool, MiruErr> {
        let (sender, receiver) = channel();
        if size == 0 {
            let e: &str = "Thread number cannot be zero!";
            return Err::<ThreadPool, MiruErr>(MiruErr::new(&e.to_string()));
        }
        else {
            let mut workers: Vec<Worker> = Vec::with_capacity(size);
            let receiver = Arc::new(Mutex::new(receiver));
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Ok(ThreadPool { workers: workers, sender: sender })
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}