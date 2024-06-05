#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Penny;

    println!("{}", value(coin));
}

fn value(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("luck fucker");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
