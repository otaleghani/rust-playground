// For libs not present in the standr library we need to link them in the 
// build.rs file. This file, found in the top directory, gives rust the
// directions of where to find the libs that it needs to link

fn main() {
    // here we specify the path to the .so file
    println!("cargo:rustc-link-search=/home/oliviero/Workspace/rust-playground/basics/ch_19_advance_features/extern_code"); 
    // here we link it
    println!("cargo:rustc-link-lib=dylib=mylib");
}
