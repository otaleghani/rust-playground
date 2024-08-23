
#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn closures_can_have_only_one_type() {
        // a simple closure that gets an item and returs it
        let example_closure = |x| x;
        let _s = example_closure(String::from("helo"));
        
        // now this closure has type string, so we cannot call it 
        // with another type like this:
        // let _n = example_closure(5);
        let _n = example_closure(5.to_string());
    }

    #[test]
    fn closure_borrow() {
        let list = vec![1,2,3];
        println!("list before the borrowing {:?}", list);

        let only_borrows = || println!("list during the borrowing {:?}", list);

        //list.push(7);
        only_borrows();
        println!("list after the borrowing {:?}", list);
    }

    #[test]
    fn closure_mut() {
        let mut list = vec![1,2,3];
        println!("list before the borrowing {:?}", list);

        let mut borrows_and_muts = || {
            list.push(7);
        };

        // println!("list during the borrowing {:?}", list);
        // note that we cannot call println after a closure definition
        // because we borrowed a mut var for list in the definition.
        // Keep always in mind that whenever you are working with closure
        // at usage you are "releasing it", but you are borrowing
        // on definition. 
        borrows_and_muts();
        println!("list after the borrowing {:?}", list);
    }
    #[test]
    fn closure_ownership() {
        let list = vec![1,2,3];
        println!("list before the borrowing {:?}", list);

        thread::spawn(move || {
            println!("list from another thread {:?}", list);
        }).join().unwrap();
    }

    // closures have 3 different traits that they can implement
    // FnOnce, a closure that can be called once
    // FnMut, a closure that doesn't move captured value out of their body
    // Fn, a closure that doesn't move nor modify the captured data
    
    // an example of FnOnce is unwrap_or_else
    // impl<T> Option<T> {
    //      pub fn unwrap_or_else<F>(self, f: F) -> T 
    //          where
    //              F: FnOnce() -> T
    //      {
    //          match self {
    //              Some(x) => x,
    //              None => f(),
    //          }
    //      }
    // }

    #[derive(Debug)]
    struct Rectangle {
        w: u32,
        h: u32,
    }

    #[test]
    fn fn_mut() {
        let mut list = [
            Rectangle { w: 10, h: 1 },
            Rectangle { w: 3, h: 5 },
            Rectangle { w: 7, h: 12 },
        ];

        list.sort_by_key(|r| r.w);
        println!("{:#?}", list);
    }
}

