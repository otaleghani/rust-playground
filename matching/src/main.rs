#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Anvedicomeballanando,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    let coin = Coin::Quarter(State::Anvedicomeballanando);

    println!("{}", value(coin));

    let x = Some(1);
    let y = plus_one(x);
    println!("{:?}", check_number(return_int(y)));
}

fn value(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("luck fucker");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn return_int(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i,
    }
}
fn check_number(x: i32) -> i32 {
    match x {
        1 => 124,
        2 => 123,
        other => 0, // The other keyword takes every OTHER possibility. This will throw a warning
                    // saying that this var is not used. You could prefix it with _ (_other) or
                    // just use _
    }
}
