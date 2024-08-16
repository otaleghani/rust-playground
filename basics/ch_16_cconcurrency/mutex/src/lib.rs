#[cfg(test)]
mod tests {
    use std::sync::{Mutex, Arc};
    use std::thread;

    // Mutex allow access to some data one thread at a time
    #[test]
    fn mutexes() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }

    // Here's how to access and modify a Mutex in multiple threads
    #[test]
    fn multithread_mutex() {
        // First off we cannot use Rc smart pointer (Reference 
        // Counter) because it's not thread safe. We'll use instead
        // Arc<T>, which is thread safe. Here we are basically 
        // giving permission to Mutex to have more than one 
        // owner across differen threads.
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        // here we spawn in this for loop 10 different threads
        for _ in 0..10 {
            // We clone the Arc<T> so that we can modify it inside
            // each thread
            let inside_counter = Arc::clone(&counter);
            
            // we then move it inside of the thread
            let handle = thread::spawn(move || {
                // and we treat it like a direct mutex, de-locking it
                // and dereferencing it to update the value
                let mut num = inside_counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        // finally we take every single JoinHandle, and we wait for
        // result of the different threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Finally we print the result of the Mutex by unlocking it 
        // and unwrapping it. 
        println!("Result: {}", *counter.lock().unwrap());
    }
}
