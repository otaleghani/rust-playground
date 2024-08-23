// static variables are like constant variables. Usually they are called
// in SCREAMING_SNAKE_CASE and can be even mutable.
// The only thing to keep in mind is that every time that you want to access
// and modify a mutable static, you'll need to do so in an unsafe scope

static HELLO_WORLD: &str = "helo";
static mut COUNTER: u32 = 0;

fn main() {
    println!("{HELLO_WORLD}");

    add_to_couter(3);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

fn add_to_couter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
