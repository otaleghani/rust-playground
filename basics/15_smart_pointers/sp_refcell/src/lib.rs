#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use crate::test::List::Cons;
    use crate::test::List::Nil;

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


    // Here we create multiple references that can mutate
    #[derive(Debug)]
    enum MutList {
        Conz(Rc<RefCell<i32>>, Rc<MutList>),
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

    // Reference cycle
    
    #[derive(Debug)]
    pub enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    #[test]
    fn reference_cycle() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("b count after chaning a = {}", Rc::strong_count(&b));
        println!("a count after chaning a = {}", Rc::strong_count(&a));
        // If you uncomment the next one it will overflow the stack
        // reason being that this two values will call each other and
        // never go out of scope. Remember that a smart pointer will 
        // never going out of scope until its strong counter goes to 0
        // println!("a next item = {:?}", a.tail());
    }

    // solving a cycle of references with Weak<T>
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[test]
    fn solving_cycles() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong = {}, weak = {}", 
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
        


        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        // Now that we associated the leaf with the parent, being that the leaf
        // has a reference to the parent, we should get an infinite loop.
        // But by using Weak, we actually stop that.
        // We can visualize it with the strong and weak count.
        println!("leaf strong = {}, weak = {}", 
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
        println!("branch strong = {}, weak = {}", 
                 Rc::strong_count(&branch),
                 Rc::weak_count(&branch));
        println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong = {}, weak = {}", 
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
        println!("branch strong = {}, weak = {}", 
                 Rc::strong_count(&branch),
                 Rc::weak_count(&branch));
    }

}
