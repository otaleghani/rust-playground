pub mod vegetable;
mod deer;
mod alpaca;
use crate::garden::deer::deerThings as hulu;
// mod deer::deerThings
// use crate::garden::deer::deer_fn;

// In the mod.rs you find the entry file where you can make public some
// methods using "pub"
pub fn helo_from_garden() {
    println!("helo from garden");
    vegetable::some_other_method();
    deer::deer_fn();
    deer::deerThings::deer_fn2();
    hulu::deer_fn2();
    alpaca::alpaca();
}
