#[cfg(test)]
pub mod function_pointer {
    fn add_one(x: i32) -> i32{
        x + 1
    }
    
    // The type fn is a type that describes a function. Basically
    // it gives you the ability to pass a function to a function
    fn do_it_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    
    #[test]
    fn function_pointer() {
        let answer = do_it_twice(add_one, 4);
        println!("{answer}");
    }
}
