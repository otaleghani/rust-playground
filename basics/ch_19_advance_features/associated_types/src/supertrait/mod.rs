use std::fmt;

// here we are saying that the trait OutlinePrint is applicable to
// every type that implements the fmt::Display trait
trait OutlinePrint: fmt::Display {
    // we define a default function for outline_print
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// we implement the fmt::Display for the Point struct
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

// and finally we impolement OutlinePrint for the Point
impl OutlinePrint for Point {}

// we can even implement this trait to other types that
// implement the fmt::Display trait.
impl OutlinePrint for String {}
impl OutlinePrint for i32 {}

// but we cannot do the same for types that do not implement
// the fmt::Display type because we cannot implement that 
// trait to them directly. But we can put this specific type
// in a custom struct inside of typle

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl OutlinePrint for Wrapper {}

#[cfg(test)]
pub mod test_supertrait {
    use super::*;
    
    #[test]
    fn trait_of_traits() {
        let p = Point { x: 1, y: 2 };
        p.outline_print();
        String::from("anvedi").outline_print();
        23.outline_print();

        let v = Wrapper(vec![String::from("anvedi"), String::from("come balla Armando")]);
        v.outline_print();
    }
}
