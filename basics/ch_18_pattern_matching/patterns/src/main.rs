fn main() {
    // If pattern
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "24".parse();

    // if let basically says, if Some(color) exists in the value var favorite_color,
    // then do the thing in the if
    if let Some(color) = favorite_color {
        println!("Using your favorite, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day madonnacane");
    } else if let Ok(age) = age {
        if age < 30 {
            println!("Using purple");
        } else {
            println!("Using orange");
        }
    } else {
        println!("Anvedi come balla nando");
    } 


    // while pattern
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // is basically the same syntaxt of if
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // for loop
    let v = vec![1,2,3];
    // In this case we use enumerate() because we want to have 
    // a tuple of index and value
    for (index, value) in v.iter().enumerate() {
        println!("index: {index}, value: {value}");
    }

    // let statement is used for deconstruction
    let (x, y, z) = (1, 2, 3);
    println!("{x}, {y}, {z}");

    // see function parameters after this
    let p = (1, 2);
    print_coordinates(&p);

    // refutability
    let some_option_value = Some(5);
    // let Some(x) = some_option_value; // This alone would not work
    // because we are not covering the possibility of a None value
    if let Some(x) = some_option_value { // By using a if let statement
                                         // if we would encounter a None we will 
                                         // just ignore this scope
        println!("{x}");
    }

    // ofcouse we cannot use this type of matching with a normal variable
    // if let x = 5 {
    //     println!("{x}");
    // }
    // it will always be true.
    
    // matching literal
    let x = 1;
    match x {
        1 => println!("first"),
        2 => println!("second"),
        3 => println!("third"),
        _ => println!("anything else"),
    }

    // named matching
    let x = Some(5);
    let _y = 10;
    match x {
        Some(50) => println!("50"),
        // The variable y doesn't refer to the y that we declared outside this match
        // in fact y is whatever variable x holds, so it will return this one
        Some(y) => println!("anvedi {y}"),
        _ => println!("default case"),
    }

    // match multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("first two numbers"),
        _ => println!("anything else"),
    }

    // match a range
    match x {
        1..=5 => println!("first few"),
        _ => println!("other ones"),
    }

    let x = 'a';
    match x {
        'a'..='j' => println!("first half of the alphabet"),
        'k'..='z' => println!("second half of the alphabet"),
        _ => println!("anvedi come balla nando"),
    }

    // Struct deconstruction
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point {
        x: 0,
        y: 6,
    };

    let Point { x: a, y: b } = p;
    println!("{a}, {b}");

    let Point { x, y } = p;
    println!("{x}, {y}");
    
    // matching a struct 
    
    match p {
        Point { x, y: 0 } => println!("is on the y axis, {x}.{y}"),
        Point { x: 0, y } => println!("is on the x axis, {x}.{y}"),
        _ => println!("is in ether"),
    }

    // enum deconstruction
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("got quit");
        },
        Message::Move { x, y } => {
            println!("got move, {x}.{y}");
        },
        Message::Write(text) => {
            println!("got write, {text}");
        },
        Message::ChangeColor(r, g, b) => {
            println!("got change color, {r} {g} {b}");
        },
    }

    // enum of enum
    enum Color {
         Rgb(i32, i32, i32),
         Hsv(i32, i32, i32),
    }
    enum NewMessage {
        ChangeColor(Color),
    }
    let new_msg = NewMessage::ChangeColor(Color::Rgb(123, 123, 123));

    match new_msg {
        NewMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("{r}, {g}, {b}");
        },
        NewMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("{h}, {s}, {v}");
        },
    }

    // deconstruction of struct and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 1, y: 2 });
    println!("{feet} {inches}, {x} {y}");

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // the _ is basically saying bind whatever
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            // here basically we are saying if both of them hold some value, do this
            println!("We cannot override an existing value.");
        },
        _ => setting_value = new_setting_value,
    }
    println!("The setted value is {:?}", setting_value);

    let s = Some(String::from("helo"));
    if let Some(_) = s {
        println!("we could even not bind a va");
    }
    // careful to put this before, because here a partial movement is performed
    if let Some(_s) = s {
        println!("Some string was found");
    }

    // Get only some fields of a struct 
    struct Point3d {
        x: i32, 
        y: i32,
        z: i32,
    }

    let origin = Point3d { x: 0, y: 0, z: 0 };
    match origin {
        Point3d { x, .. } => println!("x is {x}"),
    }

    // this is how to get certain index in a list
    let numbers = (1,2,3,4,5,6);
    match numbers {
        (first, .., last) => println!("{first}, {last}"),
    }
    match numbers {
        (first, second, .., second_last, last) => println!("{first}, {second}, {second_last}, {last}"),
    }

    // match guards
    // this is basically like creating an if statement in the match branch
    let num: Option<i32> = Some(8);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let y = 8;
    match num {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("{y}"),
        _ => println!("anvedi"),
    }


    // The @ operator lets you save a certain variable to use it afterwards
    // in the match expression. It basically binds a value in a pattern while 
    // testing it
    enum FinalMessage {
        Hello { id: i32 },
    }
    let msg = FinalMessage::Hello { id: 5 };
    match msg {
        FinalMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found this id {id_variable}"),
        FinalMessage::Hello { id: 8..=12 } => println!("Found id too high"),
        FinalMessage::Hello { id } => println!("Found with other id {x}"),
    }
}

// function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: {x}, {y}");
}


