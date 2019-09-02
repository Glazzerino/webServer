use std::thread;
use std::sync:: {
    mpsc,
    Arc,
    Mutex,
};
pub struct Threadpool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl Threadpool {
    
    pub fn new(size : usize) -> Threadpool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Threadpool{
            workers,
            sender,
        }
    }

}

struct Worker {
    id: usize,
    thread_handle: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(||{
            receiver;
        });
        Worker {
           id: id,
           thread_handle: thread,
        }
    }
}