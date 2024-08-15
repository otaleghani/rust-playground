#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    // thread::spawn excecutes the closure in another thread
    // but if the main finishes before the new thread gets terminated
    #[test]
    fn spawn_thread() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("from thread: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..5 {
            println!("from main: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    // you can create a var that handles the thread,
    // so you can wait for the thread to finish before closing main.
    // Of course whenever we use handle.join() we will stop the 
    // main thread. If we called it after the handle would be
    // defined, it would wait for the thread and then execute the rest
    // of the main
    #[test]
    fn wait_for_thread() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("thread: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..5 {
            println!("main: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }

    // moving data from the caller to the new thread requires the move keyword
    #[test]
    fn get_data_from_caller() {
        let v = vec![1,2,3];
        let handle = thread::spawn(move || {
            for i in v {
                println!("thread: {}", i);
            }
        });

        handle.join().unwrap();

        // problem with move is that we give the thread ownership of the
        // variable. So it's no longer valid in main thread here
        //  for i in v {
        //      println!("thread: {}", i);
        //  }
    }

    // To pass data from a thread to another one you'll have to use channels
    use std::sync::mpsc;

    #[test]
    fn passing_data_channels() {
        // here you define a tx (transmitter) and rx (reciever). ALWAYS
        // REMEMBER THAT RUST HAS MULTIPLE TRASMITTERS AND A SINGLE 
        // RECIEVER.
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("anvedi nando");
            thread::sleep(Duration::from_millis(1000));
            
            // You unwrap because send returns an option
            //println!("{:?}", tx.send(val)); // Gives you Ok(())
            tx.send(val).unwrap();
        });

        // You then call rx.recv to receive from the thread. This BLOCKS
        let recieved = rx.recv().unwrap();
        println!("{}", recieved);
    }


    // example of multiple transmitted messages
    #[test]
    fn multiple_messages() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("something"),
                String::from("interesting"),
                String::from("to"),
                String::from("say"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        for val in rx {
            println!("got: {}", val);
        }
    }

    // example of multiple thread transmitting messages
    #[test]
    fn multiple_messages_multiple_threads() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();

        thread::spawn(move || {
            let vals = vec![
                String::from("something"),
                String::from("interesting"),
                String::from("to"),
                String::from("say"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("something"),
                String::from("interesting"),
                String::from("to"),
                String::from("say"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        for val in rx {
            println!("got: {}", val);
        }
    }
}
