pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn stfu() {
    let guess = Guess::new(17);
    println!("{0}", guess.value);

    let rect_1 = Rectangle {
        width: 8,
        height: 9,
    };
    let rect_2 = Rectangle {
        width: 2,
        height: 3,
    };
    println!("{}", rect_1.can_hold(&rect_2));

    println!("{}", greeting("alberotne"));
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

fn greeting(s: &str) -> String {
    String::from(format!("Helo, {s}"))
}

struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 {
            panic!("This is too small, got {}", value);
        } else if value > 100 {
            panic!("This is too big, got {}", value);
        }
        Guess { value }
    }
}

// everything inside cfg(test) (cfg is configuration)
// would be run and compiled only with cargo test, not cargo run
#[cfg(test)]
mod tests {
    // this is used to get every module that is in the outer scope
    // super -> outer scope
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn rectangle_ne() {
        let rect_1 = Rectangle {
            width: 8,
            height: 9,
        };
        let rect_2 = Rectangle {
            width: 2,
            height: 3,
        };
        // fails if equal
        assert_ne!(rect_1, rect_2);
    }

    #[test]
    fn rectangle_eq() {
        let rect_1 = Rectangle {
            width: 8,
            height: 9,
        };
        let rect_2 = Rectangle {
            width: 8,
            height: 9,
        };
        // fails if not equal
        assert_eq!(rect_1, rect_2);
    }

    // #[test]
    // fn explode() {
    //     panic!("oh, no mister crab");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 9,
        };
        let smaller = Rectangle {
            width: 2,
            height: 3,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 9,
        };
        let smaller = Rectangle {
            width: 2,
            height: 3,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Carol");
        
        // Custom assert errors
        assert!(
            result.contains("Helo"),
            "Greeting does not contain the passed name.",
        )
    }

    #[test]
    #[should_panic(expected = "too big")]
    // the should_panic accepts a parameter that expects a 
    // part or the hole panic message. Here we expect that the
    // panic message would contain "too big"
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "too small")]
    fn smaller_than_1() {
        Guess::new(-2);
    }

    #[test]
    fn result_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("This got interesting"))
        }
    }

    #[test]
    #[ignore]
    // ingnores this test 
    fn long_test() {
        println!("test that we want to ignore");
    }
}

// cool flags
//
// cargo test -- --test-threads=1 | this will run the test in consecutive order, 
// usually tests are run in parallel
//
// cargo test -- --show-output | shows prints instead of capturing it
//
// cargo test name_function | runs only the test called name_function
//
// cargo test add | runs all the tests that have add in their name
//
// cargo test -- --ignored | runs only ignores tests
//
// cargo test -- --include-ignored | runs all tests, including ignored
//
//
// Important concepts
//
// there are two types of testing in rust, unit and integration tests.
// we say unit tests here: they are basically testing functions in 
// isolation and they live inside the same file of the functions that they test.
//
// integration tests are used to try things together. where unit test 
// tests the functionality in isolation, integration test tests the 
// integration of different packages together
//
// they live in the tests directory
