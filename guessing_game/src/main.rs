use std::io;
use std::cmp::ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();
    let kappa = "albertone";

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {kappa}, {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small");
        Ordering::Greater => println!("Too big");
        Ordering::Equal => println!("Nice");
    }
}
