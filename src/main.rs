use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::{fs, thread};
use std::time::Duration;
use std::thread::JoinHandle;
use std::sync::{mpsc, Mutex, Arc};
use std::borrow::Borrow;

fn main() {
    //open a socket
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    //listen for incoming traffic
    //each stream = connection attempt
    //todo take(2) only accepts 2 requests before gracefully shustting down
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("2 requests done, shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        // thread::sleep(Duration::from_secs(5)); //todo was temp
        ("HTTP/1.1. 200 OK", "src/hello.html")
    } else {
        ("HTTP/1.1. 404 NOT FOUND", "src/404.html")
    };

    //read file
    let f = fs::read_to_string(filename).unwrap();
    //response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        f.len(),
        f
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); //pauses the program until the response is written and sent to server
}

// -----------------------------------------------------------------------------

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// # Panics
    /// size must be above 0
    fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            //we're using 3 separate apis here:
            //Arc = to be able to share multiple immutable pointers for the receiver (Rc) among all the threads in a thread-safe manner (Arc)
            //Mutex = so that only one worker can mutate the receiver at a time by holding a lock
            //Mpsc = the actual channel api
            let new_worker = Worker::new(i, Arc::clone(&receiver));
            workers.push(new_worker);
        }

        Self { workers, sender }
    }

    fn execute<F>(&self, f: F) where
        F: FnOnce() + Send + 'static, //closure that executes max once, that can be sent over threat, that lives as long as the program lives in rust
    {
        let job = Box::new(f); //create a job
        let message = Message::NewJob(job); //create a message
        self.sender.send(message).unwrap(); //send a message
    }
}

// we implemented this as Drop on purpose - when threadpool goes out of scope, this drop fn is run
impl Drop for ThreadPool {
    // let's us gracefully shut down
    fn drop(&mut self) {

        //step 1 - send messages to workers to terminate (break out of loop)
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        //step 2 - actually join the threads and exit the threadpool
        //since we're modifying workers we need a mutable reference to self
        for w in &mut self.workers {
            //in order to call .join() we need to take ownership of the worker
            if let Some(thread) = w.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// -----------------------------------------------------------------------------

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>, //using Option<> here to be able to call .take() during drop
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        println!("new worker spawned!");

        //don't forget "move" here - w/o move the thread might outlive the receiver
        //also don't dorget loop - if you don't put it there the thread will close after a single request is received and processed
        //can use let here because let drops temp values at its end - which frees up lock(). if we used if let or while let - the mutex would never be dropped
        let thread = thread::spawn(move || loop {
            //lock() = acquire and hold the mutex
            //first unwrap() = panic if no job
            //recv() = part of the channel api, to receive the job from the channel
            //call to recv blocks so the thread will wait until a job is available
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a new job to execute!", id);
                    job(); //execute the job! remember we passed a function in!
                },
                Message::Terminate => {
                    println!("Worker {} exiting.", id);
                    break;
                }
            }
        });

        Self { id, thread: Some(thread) }
    }
}

// -----------------------------------------------------------------------------

//todo why a type? and not a struct?
type Job = Box<dyn FnOnce() + Send + 'static>;

// msg can be either a job or a signal to terminate
enum Message {
    NewJob(Job),
    Terminate,
}