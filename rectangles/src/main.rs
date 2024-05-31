#[derive(Debug)] // Activates debug formatting
struct Rectangle {
    width: u32,
    height: u32,
}

// This is a method. Basically a function related to an object.
// in this case is Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn area_borrow(self) -> u32 {
        self.width * self.height
    }
    // A method always has parathesis, so you could potentially have a method with the same name of
    // a parameter of the object.
    // The difference lies in where or not you are adding parenthesis at the end.
    // struct.field     -> field
    // struct.field()   -> method
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect = Rectangle {
        width: dbg!(30 * 2),
        height: 50
    };

    println!("Area: {}", area(width1, height1));
    println!("Area: {}", area_tuples(rect1));
    println!("Area: {}", area_struct(&rect));
    dbg!(&rect);
    println!("Rect: {:#?}", rect); 
    // Prints the rect with pretty print, either {:?}
    // or {:#?}. This needs the debug formatting
    println!("Area: {}", rect.area()); // Instance.Method(params...)
    println!("Area: {}", rect.area_borrow()); // this is the same function, but this time we are
                                              // not borrowing, we are taking ownership
    // ence calling this... println!("Rect: {:#?}", rect); will actually crash our program
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
