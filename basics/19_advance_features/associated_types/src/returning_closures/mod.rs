#[cfg(test)]
pub mod returning_closures {
    // Closures are not returnable directly. But you just need to put them
    // in a box and you're fine.
    #[test]
    fn returning_closures() {
        let x = return_closure();
        println!("{}", x(5));
    }

    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| {
            x + 1
        })
    }
}
