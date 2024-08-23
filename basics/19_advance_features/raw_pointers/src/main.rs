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
    // Lets say for example that we want to create a function that takes in a slice
    // and returns two slices, one the first half and the other the second half 
    
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

// to call unsafe functions you need to call them in an unsafe scope
unsafe fn dangerous() {
    println!("im doint dangerous things here");
}

use std::slice;

#[allow(dead_code)]
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);

    // This function would not work because you have two mutable borrows
    // and only one at a time is allowed
    // (&mut values[..mid], &mut values[mid..])

    // This let's us access the raw pointer of the slice
    let ptr = values.as_mut_ptr();

    // we can now use this raw pointer to return part of the slice
    // remember that the pointer of the slice points to the first value
    unsafe {
        (
            // here we are taking from the pointer to mid
            slice::from_raw_parts_mut(ptr, mid),

            // here we are accessing from the pointer + mid to the length
            // - mid. Reason why we do this is because we want to travers
            // from the ptr to the mid ptr and then we want to take the 
            // length of the hole slice minus the mid ptr part
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),

            // we are wrapping everything in unsafe because the from_raw_parts_mut
            // and add methods are unsafe.
        )
    }
}
