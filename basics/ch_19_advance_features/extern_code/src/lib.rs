// imagine now that you want to import a rust function inside another program.
// to do this you need to do a couple of steps

// no mangle means that the function will retain its name
#[no_mangle]
// here we are making public this function for C
pub extern "C" fn call_from_c() {
    println!("Calling Rust from C");
}
