// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// // You can import parent creates, useful if you want to import 
// // crates that haave the same names
// use std::fmt;
// use std::io;
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result {}
// // Like here if you try to add std::fmt::Result and std::io::Result
// // rust will not know where to get what
// // You could have used the 
// use std::fmt::Result as fmtResult 
// // Which is a-okay
// // You could even re-export a named var with pub
// // THINGS TO UNDERSTAND
// // everything is private by default, pub makes it public (reachable
// // and usable) mod defines a crate. You use "mod crate" to

mod front_of_house {
    pub fn alberto() {
        println!("alberto");
    }

    pub mod hosting {
        pub fn add_to_waitlist() {
            crate::front_of_house::alberto();
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist(); // relative path
}
