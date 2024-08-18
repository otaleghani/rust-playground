// associated types connect a type placeholder with a trait such that the
// trait method definition can yse these placeholder in their signatures

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// here we are implementing the + operation to the Point struct.
// This is also called OPERATOR OVERLOADING
impl Add for Point {
    // To do this we need to change the type Output to the actual type that we are
    // using now. The "type Output" is basically a type placeholder that we'll use 
    // to add the + operation to this strut.
    type Output = Point;

    // and here we implement the + operation
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    
    // The trait Add is defined as <Rhs=self>, so it means that we can change the
    // Right Hand Side of the operation to be another type instead of self.
}

struct Millimeters(u32);
struct Meters(u32);

// here we are basically creating an operator overload for millimeters + meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        // here we are ofc saying that the other has to be * 1000
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub trait Nando {
    type Item;
    fn anvedi_come_balla(&self) -> Self::Item;
    fn count_the_total(&self) -> i32;
}

impl Nando for Point {
    type Item = Point;

    fn anvedi_come_balla(&self) -> Point {
        println!("Avedi come balla nando");
        *self
    }

    fn count_the_total(&self) -> i32 {
        println!("I'm counting the total in this point ");
        self.x + self.y
    }
} 

fn main() {
    let p1 = Point { x: 2, y: 2 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1 + p2;
    println!("{:?}", p3);

    let p4 = p3.anvedi_come_balla();
    println!("{:?}", p4);
    println!("{}", p4.count_the_total());

    let mm = Millimeters(1000);
    let m = Meters(1);
    println!("{}", (mm + m).0);
}
