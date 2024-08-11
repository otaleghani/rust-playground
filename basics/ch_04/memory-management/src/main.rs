fn main() {
    // Remember, you have the stack and the heap
    // stack is fast, holds less data, access as a stack
    // heap is slow, holds more data, access by memory point

    // Memory rules
    // 1. Rust doesn't create deep copies by default
    // 2. Rust doesn't create shallow copies (just copying the memory
    //    address), instead it moves the data
    // 3. Rust will drop every variable that goes out of scope with
    //    the `drop` method
    // 4. When you copy a variable from the heap you will
    //    automatically drop the original value

    let s1 = String::from("hello");
    // let s2 = s1;
    // So after s2 = s1, s1 is no longer valid
    // println!("{s1}"); // Will throw an error

    let s2 = s1.clone();
    // This is how you'll create a deep copy, heap data gets copied
    // into the s2 variable
    println!("s1: {s1} | s2: {s2}"); // Will not throw an error
    
    // With rust primitives whenever you do something like this you
    // are actually copying the data inside the stack. This is
    // actually a deep copy by default, because is faster.
    let x = 5;
    let y = x;
    println!("x: {x} | y: {y}"); // Will throw an error
    
    // Let's put the ownership concepts to the test
    let s = String::from("im a shrimp");
    println!("{s}");

    takes_ownership(s);
    // println!("This will throw an error, because {s} is out of the scope");
    // But the same cannot be said of a primitive, like uint
    let x = 1;
    println!("{x}");
    takes_ownership_int(x);
    println!("But I'm still here: {x}");
    
    // Here we'll try something interesting
    // The return values give ownership, so this one gives you the
    // ownership of a value to st1
    let st1 = gives_ownership();
    // st1 is then dropped because it's moved to st2
    let st2 = st1;
    // that finally gives it's data to get the data to st3
    let st3 = takes_and_gives(st2);
    // You could have said something like let st2 =
    // takes_and_gives(st2); to re-reference st2
    println!("{st3}");

    // multiple returns example
    // As you can see you can even re-allocate variables that has been
    // dropped
    let st1 = String::from("some string");
    let (st2, len) = calculate_length(st1);
    println!("The string {st2} has length {len}");

    // References, basically pointers that you can use to reference a
    // value instead of giving up the ownership. A reference is
    // created by adding an & before the variable.
    let st1 = String::from("some other loooooooooong string");
    let len = calculate_length_reference(&st1);
    println!("The string {st1} has length {len}");

    // here we are passing a mutable reference, so basically this var
    // can be changed by this function. It's really clear that it will
    // change the value because it has &mut both here and in the
    // function signature
    // KEEP IN MIND, you can multiple borrows, but ONLY ONE MUTABLE
    // BORROW ACTIVE. So if you have
    //
    // let s = String::from("sometext")
    // let x = &s
    // let y = &s
    // 
    // It will compile no problem. But
    //
    // let s = String::from("sometext")
    // let x = &s
    // let y = &s
    // let k = &mut s
    //
    // Will throw an error because if you have a mutable borrow the
    // readers (&s) could have the data suddently change under them.
    // This is a data race, and Rust will not compile that.
    let mut st2 = String::from("some string");
    change(&mut st2);
    println!("{st2}");

    // Also keep in mind the fact that if a var goes out of scope it
    // will be dropped. You cannot reference a val that has beed
    // dropped, reason why is because the reference points to an area
    // in memory that doesn't have your expected data.
    //
    // If you are closing a scope and want to keep the data, you'll
    // have to allocate it to another var by returning it.
}

fn takes_ownership(some_string: String) {
    println!("I've taken ownership of this value: {some_string}");
}
fn takes_ownership_int(some_int: i32) {
    println!("I've taken ownership of this value: {some_int}"); 
}
fn gives_ownership() -> String {
    let some_string = String::from("cannarsi");
    some_string
}
fn takes_and_gives(some_string: String) -> String {
    some_string
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// Here we are specifying that we want a reference to a String
// This function never has the ownership of s, so the caller s will
// never be dropped. The action of creating a reference is called
// BORROWING. Keep in mind that we cannot change the borrowed value.
// For doing it we will need a mutable reference
fn calculate_length_reference(s: &String) -> usize { 
    s.len()
}

// Example of mutable borrowed value
fn change(s: &mut String) {
    s.push_str(", incredibbile");
}
