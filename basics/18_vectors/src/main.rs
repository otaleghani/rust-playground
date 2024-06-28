fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    // let v = vec![1,2,3];
    let third: &i32 = &v[2];
    let second: &i32 = &v[1];
    let first: &i32 = &v[0];
    // v.push(4) will cause a crash, cause you have references to 
    // 1,2,3 
    println!("{third}, {second}, {first}... Go!");
    v.push(4);

    // Get an element with indices with .get for a safe method
    let number: Option<&i32> = v.get(3);
    match number {
        Some(number) => println!("The number is {number}"),
        None => println!("No number found. Not wholesome chungus"),
    }
    
    for i in &v {
        println!("{}", *i*2);
    }

    // Vector have only one type, but that type could be an enum!
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Insert in index and shifts everything to the right
    println!("\nINSERTING:");
    row.insert(0, SpreadsheetCell::Int(4));
    
    for i in &row {
        println!("{:?}", i);
    }

    // pop last element
    println!("\nPOPPING:");
    let popped = row.pop();
    match popped {
        Some(popped) => println!("{:?}", popped),
        None => println!("None found"),
    };

    // remove removes index and then shifts to the left
    println!("\nREMOVING:");
    row.remove(0);
    
    for i in &row {
        println!("{:?}", i);
    }

    // truncate keeps just n vector, drops the rest
    println!("\nTRUCATING:");
    row.truncate(1);

    for i in &row {
        println!("{:?}", i);
    }

    // clear deletes everything
    println!("\nCLEARING:");
    row.clear();

    println!("len of row: {}", row.len());

    // Look at the documentation for everything else.
}
