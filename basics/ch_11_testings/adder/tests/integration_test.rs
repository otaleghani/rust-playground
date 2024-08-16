use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    common::another::another_setup();
    assert_eq!(4, adder::add(2,2));    
}
// this is an integration test. In this case we don't need 
// #[cfg(test)], because in the tests dir everything is a test
//

// cool flags
// cargo test --test integration_test | runs only the specified integration test
