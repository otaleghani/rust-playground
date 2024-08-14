#[cfg(test)]
mod test {
    use std::rc::Rc;

    // The Rc<T> let's you share immutable data with different owners.
    // The data remains valid until there are no more owners 

    #[derive(Debug)]
    #[allow(dead_code)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    #[test]
    fn rc_xample() {
        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        let b = List::Cons(3, Rc::clone(&a));
        let c = List::Cons(4, Rc::clone(&a));

        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
        // As you can see here both b and c have a reference to a, 
        // they both can read that no problems
    }

    // Rc stands for Reference Count, so ofc we can count the amount of
    // references this thing has. When the amount reaches 0 the value
    // is freed from memory.
    #[test]
    fn rc_count() {
        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        println!("The number of references to a is {}", Rc::strong_count(&a));

        let b = List::Cons(3, Rc::clone(&a));
        println!("The number of references to a is {}", Rc::strong_count(&a));

        let c = List::Cons(4, Rc::clone(&a));
        println!("The number of references to a is {}", Rc::strong_count(&a));

        // Here everything goes out of scope so the count becomes 0 in an instant
        // we could drop it tho
        drop(b);
        println!("The number of references to a is {}", Rc::strong_count(&a));

        drop(c);
        println!("The number of references to a is {}", Rc::strong_count(&a));
    }

    // RefCell<T> is another smart pointer to take into consideration.
    // This one is really interesting because it makes you circumvent the
    // borrow checker rules, enforcing them at runtime instead that are compile time.
    //
    // This is really good expecially whenever you want to create mock objects to 
    // test functionality of certain parts of your code.

    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger>{
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T> 
    where 
        T: Messenger, 
    {
        pub fn new(
            messenger: &'a T,
            max: usize
        ) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota.");
            } else if percentage_of_max >= 0.9 {
                self.messenger.send("Urgent: You are at 90% capacity");
            } else if percentage_of_max >= 0.75 {
                self.messenger.send("Warning: You are at 75% capacity");
            }
        }
    }


    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[derive(Debug)]
    enum MutList {
        Conz(Rc<RefCell<i32>>, Rc<List>),
        Nill,
    }

    #[test]
    fn multiple_mut_owners() {
        use crate::test::MutList::Conz;
        use crate::test::MutList::Nill;

        let value = Rc::new(RefCell::new(5));
        let az = Rc::new(Conz(Rc::clone(&value), Rc::new(Nill)));
        let bz = MutList::Conz(Rc::new(RefCell::new(3)), Rc::clone(&az));
        let cz = MutList::Conz(Rc::new(RefCell::new(4)), Rc::clone(&az));

        *value.borrow_mut() += 10;
        println!("a after = {:?}", az);
        println!("b after = {:?}", bz);
        println!("c after = {:?}", cz);
    }

}
