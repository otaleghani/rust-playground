fn main() {
    // Creates a new empty string
    let mut s = String::new();
    // let mut s = "some data that's not encoded".to_string();
    // let mut s = String::from("some string");
    s.push_str("foo");

    let s2 = String::from("bar");
    // Taks in a string slice, not String
    s.push_str(&s2);
    // So we have still s2 at our disposal

    println!("{s}");
    println!("{s2}");

    let s = String::new();
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s + &s1 + &s2;
    // s1 is not gone, s3 took ownership
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // In rust you cannot access an index of a string
    // reason is that rust will return the "value", not 
    // the actual letter. So "h" would be 104
    // Also there are grapheme clusters: these are like in
    // the accents, but instead of being in a separate char,
    // they are like addable to a char and they take up one char.
    // So instead you will use string slices
    
    println!("{}", &s[0..1]);
    // always keep in mind that sometimes there are chars
    // that take up 2 bytes, like the russian chars 
    // Accessing one of the two bytes will crash the program
    

    // You can access both the single chars of a strings or
    // the bytes if you need 
    for c in "anvedi".chars() {
        println!("{c}");
    }
    for c in "anvedi".bytes() {
        println!("{c}");
    }
}
