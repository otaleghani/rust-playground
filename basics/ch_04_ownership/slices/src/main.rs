fn main() {
    let mut s = String::from("hello world!");
    
    // word will have value 5
    let _word = first_word(&s);

    // But if we clear the array word will not be valid. clear()
    // empties a string.
    s.clear(); 

    // Imagine creating another function, this time for getting the
    // second word. It will be chaos. You can use slices to fix this
    // problem.
    s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // Slices hold the pointer to the starting_index of the original
    // data and have a length value, given by ending_index - starting_index
    println!("{hello} {world}");

    // They look like golang slices, and feel like them, cause you can
    // do something like this.
    let hello = &s[..5];
    let world = &s[6..];
    let hello_world = &s[..];
    println!("{hello} {world}");
    println!("{hello_world}");

    // With slices we can go back to the original function and make it
    // better by using slices, because it will catch at compile time
    // the fact that we are trying to accessing an out of range index
    let first_word = first_word_better(&s);
    // s.clear(); will throw an error this time because s.clear()
    // needs a mutable reference to s to actually do it's thing. And
    // we cannot have both a mutable and immutable reference in one
    // single code block.
    println!("{first_word}");


}

fn first_word(s: &String) -> usize {
    // This function returns the index where the first word in an
    // array ends

    // converts String to bytes
    let bytes = s.as_bytes();

    // iter() iterates for every byte and enumerate wraps the result
    // of iter as part of a tuple. 
    // iter()           -> iters every byte
    // enumerate()      -> returns index e value of iter
    // (i, &item)       -> deconstruction of the result of enumerate
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_better(s: &String) -> &str {
    // This returns a string literal (&str)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
