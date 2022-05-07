use quill::{PluginLogger};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        PluginLogger::info("WebsitePlugin", "Sending terminate message to all workers.".to_string());

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        PluginLogger::info("WebsitePlugin", "Shutting down all workers.".to_string());

        for worker in &mut self.workers {
            PluginLogger::info("WebsitePlugin", format!("Shutting down worker {}", worker.id));

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    PluginLogger::info("WebsitePlugin", format!("Worker {} got a job; executing.", id));

                    job();
                }
                Message::Terminate => {
                    PluginLogger::info("WebsitePlugin", format!("Worker {} was told to terminate.", id));

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