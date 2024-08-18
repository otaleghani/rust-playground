// extern let's you call other functions from other programming languages
extern "C" {
    fn abs(input: i32) -> i32;
}

// here we are linking to this program a custom lib called mylib,
// written in C. The only thing that you need to do when creating a link
// like this is defining the fn signature like it was rust. This is actually
// givin to rust the ability to interface with this external function
#[link(name = "mylib")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        let result = add(3, 4);
        println!("Result of addition: {}", result);
    }
}
