use crate::garden::vegetable::Leek;
pub mod garden;
mod house;
// mod let's you import something

// A crate can be of 2 types, binary and library
// A create can have many binaries (in the /src/bin/*.rs)
// and only one library (in the /src/lib.rs
fn main() {
    println!("Hello, world!");
    garden::helo_from_garden();
    let new_leek = Leek {
        name: String::from("samdro"),
    };
    println!("{}", new_leek.name);
    house::check_on_brat();
}
