struct Rectangle {
    width: u32,
    height: u32,
}

// Everything defined inside of this impl block is called an
// associated function
impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width < rect.width && self.height < rect.height
    }
    
    // You can have functions that are related to the struct, but
    // don't have a struct as a parameter. They usually are
    // constructur type of functions.
    // You call them with "::" instead of a ".", and you use the name
    // of the struct. You can even use Self to reference the struct
    fn square(i: u32) -> Self {
        Self {
            width: i,
            height: i,
        }
    }

    fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };
    let rect3 = Rectangle::new(20, 30);
    let square1 = Rectangle::square(20);
    println!("Can 1 hold 2? {}", rect1.can_hold(&rect2));
    println!("Can 2 hold 1? {}", rect2.can_hold(&rect1));
    println!("The new rectangle has {} width and {} height", rect3.width, rect3.height);
    println!("The square is {}", square1.width);
}
