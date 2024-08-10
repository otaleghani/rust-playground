use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    //_error_handling_matcth()
    //_error_handling_unwrap()
    //_error_handling_except()
    let file_data = _question_operator();
    match file_data {
        Ok(result) => println!("{result}"),
        Err(error) => println!("{:?}", error),
    }
}

// whenever we want to manage error we could return a Result<T,E>
// T is the actual data
// E is the error
// Is kinda like Option<Some,None>
//
// The File::open operation returns either the file handle or 
// the error, that can be analyzed using io::ErrorKind
// Usually you can use a match to execute operation after the opening of this file
fn _error_handling_match() {
    let _greeting_file_result = File::open("hello.txt");
    let _greeting_file = match _greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problme creating the file: {:?}", e),
                }
            }
            other_error => {
                panic!("Problme opening the file: {:?}", other_error);
            }
        },
    }; 
}

// Sometimes match could be a little to verbose. For instance you could use 
// unwrap_or_else, that gives you the file handle or executes something if it
// errors
fn _error_handling_unwrap() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// Sometimes you just want that a function just executes without problems
// so you want to panic if it errors. For this you just want to use except
// that returns the function value, but adds the ability to add a message
// for context if the result errors
fn _error_handling_except() {
    let _greeting_file = File::open("hello.txt").expect("hello.txt is required for this project");
}

// error propagation is similar of what you find in golang
// In rust you propagate errors using the Result type
fn _error_propagation() -> Result<String, io::Error> {
    let _greeting_file_result = File::open("hello.txt");
    let mut username_file = match _greeting_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// an even shorter version of error propagation.
// this time using the ? operator, that returns Err if an
// error occurs
fn _question_operator() -> Result<String, io::Error> {
    let mut _username_file = File::open("hello.txt")?;
    let mut username = String::new();
    _username_file.read_to_string(&mut username)?;
    Ok(username)
}

// an even shorter version
fn _question_operator_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// an important thing to understand is that the ?
