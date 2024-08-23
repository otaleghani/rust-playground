const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // Mutable variable

    println!("Also, do you know that tree hours are equal to
             {THREE_HOURS_IN_SECONDS} seconds?\n");
    // Constant

    let y = 5;
    let y = y + 1;
    {
        let y = y + 5;
        println!("The value of y is: {y}");
    }
    // An inner scope

    println!("The value of y is: {y}");
    // Shadowing
    //
    // Remember: Shadowing is done using always "let", you can change
    // the type of the data.
    
    let z: f64 = 2.0;
    let k = 3.0;
    let sum = z + k;

    println!("Sum is {sum}");
    // You cannot use different types. 

    let z: f64 = 2.0;
    let k: f64 = 40.0;
    let sum = z + k;
    let dif = z - k;
    let mul = z * k;
    let div = z / k;
    let rem = z % k;
    println!("Sum: {sum}\nDif: {dif}\nMul: {mul}\nDiv: {div}\nRem: {rem}");

    let z = true;
    println!("{z}");
    let z: bool = false;
    println!("{z}");

    let z = 's';
    println!("{z}");
    let z: char = 'q';
    println!("{z}");

    let z: (u32, f32, char) = (1, 1.1, 'c');
    let (a, b, c) = z;
    println!("{a}, {b}, {c}");
    let a = z.0;
    let b = z.1;
    let c = z.2;
    println!("{a}, {b}, {c}");
    // let z = ();
    // Empty tuple are usually used as return types

    let _z = [1,2,3,4,5];
    // arrays are fixed in size
    let _z: [i32; 5] = [1,2,3,4,5];
    // You can define the lenght it in the type definition
    let z = [2; 5];
    // You can even initialize the array with a value, in this case 2
    let a = z[0];
    println!("{a}");

    // If you try to access a out of range index, rust will just panic
    // instead of giving you the ability to read memory outside the
    // program. One of the security features of Rust over C.

    new_function(32, 'a');

    // expressions return a value
    // statements do not.
    
    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");

    let z = return_function();
    println!("{z}");
}

fn new_function(x: i32, k: char) {
    println!("Im in new_function {x}, {k}");
    // rust does not care about the position of the function. It can
    // be defined before the main, after the main. It just has to be
    // visible to the caller.
}

fn return_function() -> i32 {
    5 + 5
    // This is kinda like an interesting thing. Basically you are
    // returning an expression. But you can also return before with a
    // return keyword.
}
