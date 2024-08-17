fn main() {
    let mut num = 5;

    // raw pointers are basically normal C pointers. They do not enforce the borrow checker,
    // the are not guaranteed to point to valid memory, the are allowed to be null and
    // they don't do automatic cleanup.
    //
    // immutable raw pointer
    let r1 = &num as *const i32;  

    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    // To use the raw pointers you need to wrap your code in an unsafe scope
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        dangerous();
    }

    // raw pointers and unsafe code is really useful whever you want to do some 
    // operations that require to have more than one variable accessing a mut pointer 

}

// to call unsafe functions you need to call them in an unsafe scope
unsafe fn dangerous() {
    println!("im doint dangerous things here");
}
