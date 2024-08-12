use std::fmt::Display;

// lifetime annotation for a struct that holds a reference,
// and not a hole value
struct ImportantExcept<'a> {
    part: &'a str,
}

fn main() {
    //not_long();
    //long_enough();
    
    let string1 = String::from("asdf");
    let string2 = String::from("xyz");
    let mut result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("{result}");

    let novel = String::from("Call me Ishin Ashina...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a dot");
    let i = ImportantExcept {
        part: first_sentence,
    };
    println!("{0}", i.part);

    result = longest_the_second(string1.as_str(), string2.as_str(), "anvedi".to_string());
    println!("{result}");
}


// wont compile because y goes out of scope, 
// so the reference hold in x is invalid
//fn not_long() {
//    let x;
//    {
//        let y = 4;
//        x = &y;
//    }
//    println!("{x}");
//}

// will compile because y doent go out of scope
fn _long_enough() {
    let x;
    let y = 4;
    x = &y;
    println!("{x}");
}

// wont compile because the compiler doesnt know the 
// reference to return. It could be x or y, but we 
// dont know it at compile time. we need a named
// lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// We have a lot of annotation to do with generics, traits
// and lifetimes. Let's put them all together
fn longest_the_second<'a, T>(
    x: &'a str, y: &'a str, ann: T
) -> &'a str
where 
    T: Display,
{
    println!("Announcement: {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
