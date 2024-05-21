fn main() {
    let number = 3;
    
    // The classical if else construction
    if number < 5 {
        println!("condition was true");
    } else if number == 7 {
        println!("condition was false");
    } else {
        println!("why am I here");
    }
    
    // always remember that if is an expression
    // the expressions inside of the if else need to be the same type,
    // else it will throw an error:
    let _x = if number == 4 { 4 } else { 5 };
    let x = if number == 4 { "sandro" } else { "seppia" };
    // This will make the program panic, because it will not know in
    // advance the type of number
    // let x = if number == 4 { 4 } else { "seppia" };
    println!("{x}");
}
