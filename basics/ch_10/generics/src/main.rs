// struct with generics
// Remember, T has to be the same type. If you want more types
// you'll need to define them.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct _NewPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

// If you have a generic you ALWAYS have to declare it in
// struct, enum, impl and methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl<X1, Y1> _NewPoint<X1, Y1> {
    fn mix_up<X2, Y2> (
        self,
        other: _NewPoint<X2, Y2> 
    ) -> _NewPoint<X1, Y2> {
        _NewPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// We already saw a couple of enum that use generics
#[derive(Debug)]
enum _Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum _Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![1,2,3,4];
    let result = _largest(&number_list);
    println!("{result}");

    let char_list = vec!['a','b','c','d'];
    let result = _largest(&char_list);
    println!("{result}");

    let p = Point {x: 2, y: 4};
    println!("x: {}", p.x());
    println!("y: {}", p.y());

    let p1 = _NewPoint {x: 2, y: 4};
    let p2 = _NewPoint {x: 2.1, y: 4.2};
    println!("y: {:?}", p1.mix_up(p2));
}

// function with generics. in this case we have to
// add to this generic this PartialOrd, because we are
// using the greater than operator, which is not used
// by all types.
fn _largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

