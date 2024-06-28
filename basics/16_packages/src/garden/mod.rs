use rand::Rng;                  // use external crate
// use std::{cmp::Ordering, io};   // import different package from same crate
// use std::io::{self, Write};     // import std::io and io::Write
// use std::collections::*;        // glob operator
pub mod vegetable;
mod deer;
mod alpaca;
use crate::garden::deer::deerThings as hulu;
// mod deer::deerThings
// use crate::garden::deer::deer_fn;
use grass::touch as lava;

// In the mod.rs you find the entry file where you can make public some
// methods using "pub"
pub fn helo_from_garden() {
    println!("helo from garden");
    vegetable::some_other_method();
    deer::deer_fn();
    deer::deerThings::deer_fn2();
    hulu::deer_fn2();
    alpaca::alpaca();
    // lava();
    grass::touch();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);
} 

mod grass {
    pub fn touch() {
        println!("don't be a fool, you are allergic to grass.");
    }
}
