#[cfg(test)]
mod test {
    #[test]
    fn vectors() {
        // Create an empty vector (type must be known)
        let mut _a: Vec<i32> = Vec::new();
        let mut _b: Vec<i32> = vec![];

        // Creating a vector with some data inside
        let mut _a = vec![1,2,3];            // Types are infered
        let mut _b: Vec<i32> = vec![1,2,3];  // Type declared
        let mut a = vec![1i32, 2, 3];       // Type suffixed on the first element
        let mut b = vec![0; 10];            // Create a vec with 10 zeros inside

        // Add item in a vec
        a.push(1);
        b.push(1);

        println!("{:?}", a);
        println!("{:?}", b);

        // Pop item in an element
        let element = match a.pop() {
            Some(i) => println!("{i}"),
            None => println!("Got nothing"),
        };
        println!("{:?}", a);

        let e = b.pop();
        let element = &e.unwrap() + 1;
        println!("{:?}", e);
        println!("{:?}", element);

        // Always remember, a vector is a pointer to the first element of the vector. 
        // It also stores the length of the vector and the max capacity of the vector.
        let l = b.len();
        let c = b.capacity();
        println!("Length: {}, Capacity: {}", l, c);

        // Always remember, the capacity of a vector gets auto resized to accomodate
        // the amount of data. This process can be slow at times. An optimization to do
        // is to allocate a given capacity when defining the variable
        let c: Vec<i32> = Vec::with_capacity(10);
        println!("Length: {}, Capacity: {}", c.len(), c.capacity());

        // Vector can be used with iterators
        for i in &b {
            print!("{i} ");
        }
        println!("");

        for i in &mut b {
            *i += 1;
            print!("{i} ");
        }
        println!("");

        for i in b.iter() {
            print!("{i} ");
        }
        println!("");

        for (i, v) in b.iter().enumerate() {
            print!("v: {v}, i: {i}");
        }
        println!("");
    }
}
