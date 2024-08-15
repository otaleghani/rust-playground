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
}
