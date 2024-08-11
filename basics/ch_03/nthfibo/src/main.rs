use std::io;

fn main() {
    // get n

    println!("GIMME YOUR NUMBER: ");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("ERROR: {error}");
            return;
        }
    };
    
    let mut result = 0;
    let mut before = 1;
    let mut index = 0;
    while index < x {
        println!("Fib pos {index}={result}");
        result = result + before;
        before = result;
        index += 1;
    } 
}
