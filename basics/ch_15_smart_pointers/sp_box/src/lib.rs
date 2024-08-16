// A box is a smart pointer. A smart pointer is just a pointer with extra features. 
// The box let's allocate data into the heap instead of the stack.
//
// Advantages:
// - the data that you want to store is not known at compile time.
// - the data is really large, so you want to point and not copy
// - you want to have a var with a specific trait instead of being of a specific value
 
#[cfg(test)]
mod tests {

    #[test]
    fn simple_box() {
        let x = Box::new(5);
        println!("x = {x}");
    }

    // Imagine you have a linked list
    enum _List {
        Cons(i32, List),
        Nil,
    }
    // problem with this enum is that its recursive, so you cannot know the size of it
    // To fix a problem like this you need to store it in the heap instead of the stack, so 
    // it can grow as much as the lists wants
    #[derive(Debug)]
    #[allow(dead_code)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    #[test]
    fn box_hold_addresses() {
        let list = List::Cons(
            1, 
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    3,
                    Box::new(List::Nil)
                ))
            ))
        );
        println!("{:?}", list);
    }

    #[test]
    // We can reference 
    fn dereferencing() {
        let x = 5;
        let y = &x;
        println!("{y} {} {x}", *y);
        assert_eq!(5, x);
        assert_eq!(5, *y); 
        // This will not work becayse y is a reference and you 
        // need to dereference it with *
        // assert_eq!(5, y);
    }

    // The Box<T> is just a tuple with one argoument
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        } 
    }

    // You can basically create your custom smart pointer.
    // Like in this case we created a new struct called mybox
    // that extends the trait Deref with a custom implementation.
    // Now MyBox can be dereferenced
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello, {name}");
    }
    #[test]
    fn dereference_custom_type() {
        let m = MyBox::new(String::from("anvedi oh"));
        hello(&m);
    }

    // Another important thing that a smart pointer should have is the 
    // drop function. We can implement it with Drop and adding the fn
    // drop to out custom type.
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data {}", self.data);
        }
    }

    #[test]
    fn dropping_a_value() {
        let c = CustomSmartPointer {
            data: String::from("anvedi oh che gnocca"),
        };
        println!("We now have a CustomSmartPointer with data {}", c.data);
        // Here CustomSmartPointer would go out of scope. Before doing that
        // the custom implementation of drop prints a message
        
        // remember that we cannot call the c.drop() method directly if the 
        // drop method is custom. We should use the drop(c) method instead
        drop(c);
    }
}
