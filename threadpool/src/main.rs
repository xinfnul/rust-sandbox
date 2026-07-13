#![forbid(unsafe_code)]

mod download;

use std::{
    fs,
    path::PathBuf,
    sync::{
        Arc, Mutex, PoisonError,
        mpsc::{self, SendError},
    },
    thread::{self, JoinHandle},
};

use crate::download::download_image;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThreadpoolError {
    MutexPoisoned,
    ThreadpoolShutdown,
    InvalidPoolSize,
}

impl std::fmt::Display for ThreadpoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPoolSize => {
                write!(f, "thread pool size must be greater than zero")
            }
            Self::MutexPoisoned => {
                write!(f, "worker receiver mutex was poisoned")
            }
            Self::ThreadpoolShutdown => {
                write!(
                    f,
                    "thread pool has been shut down and cannot accept new jobs"
                )
            }
        }
    }
}

impl std::error::Error for ThreadpoolError {}

impl<T> From<PoisonError<T>> for ThreadpoolError {
    #[inline]
    fn from(_: PoisonError<T>) -> Self {
        Self::MutexPoisoned
    }
}

impl<T> From<SendError<T>> for ThreadpoolError {
    #[inline]
    fn from(_: SendError<T>) -> Self {
        Self::ThreadpoolShutdown
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = {
                    let receiver = match receiver.lock() {
                        Ok(r) => r,
                        Err(e) => {
                            eprintln!("Worker {}: {}", id, ThreadpoolError::from(e));
                            break;
                        }
                    };

                    receiver.recv()
                };

                match message {
                    Ok(job) => job(),
                    Err(_) => {
                        #[cfg(debug_assertions)]
                        println!("Worker {id} shutting down.");

                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[must_use]
struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<Self, ThreadpoolError> {
        if size == 0 {
            return Err(ThreadpoolError::InvalidPoolSize);
        }

        let (tx, rx) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(tx),
        })
    }

    #[inline]
    pub fn execute<F>(&self, f: F) -> Result<(), ThreadpoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let sender = self
            .sender
            .as_ref()
            .ok_or(ThreadpoolError::ThreadpoolShutdown)?;

        sender.send(Box::new(f))?;

        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                if thread.join().is_err() {
                    eprintln!("Worker {} panicked.", worker.id);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("images")?;

    let pool = ThreadPool::new(8)?;

    let client = Arc::new(reqwest::blocking::Client::new());

    for i in 1..200 {
        let client = Arc::clone(&client);

        let url = format!(
            "https://yavuzceliker.github.io/sample-images/image-{}.jpg",
            i
        );

        let path = PathBuf::from(format!("images/image-{}.jpg", i));

        pool.execute(move || match download_image(&client, &url, &path) {
            Ok(_) => println!("Downloaded image {}", i),
            Err(e) => eprintln!("Failed to download image image {}: {}", i, e),
        })?;
    }

    Ok(())
}
