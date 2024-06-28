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
}
