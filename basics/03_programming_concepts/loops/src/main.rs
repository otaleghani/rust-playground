fn main() {
    // The classic inifite for
    // loop {
    //     println!("SANDRO");
    // }
    
    let mut counter = 0;
    // Here you are using a loop to create a value
    let result = 'supersonico: loop {
        counter += 1;
        if counter == 10 {
            // You return the data using break
            loop {
                counter += 1;
                if counter == 50 {
                    // labels
                    break 'supersonico counter * 2;
                };
            };
        };
    };
    println!("{result}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Im out of the while loop");

    let a = [1,2,3,4,5];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("the value is: {}", element);
    }
    for element in (1..4).rev() {
        println!("the value is: {}", element);
    }
}
