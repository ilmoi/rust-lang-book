use std::sync::{Mutex, Arc};
use std::thread;

fn main() {

    let m = Arc::new(Mutex::new(5)); //thread-safe version of Rc<T>

    let m_clone = Arc::clone(&m); //clone it before moving into 2nd thread

    let handle = thread::spawn(move || { //"move" needed to move m_clone to 2nd thread
        let mut num = m_clone.lock().unwrap();
        *num = 7;
    });

    handle.join().unwrap();

    println!("m is {:?}", m);

}