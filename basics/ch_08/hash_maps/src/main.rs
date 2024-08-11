use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");

    // .get -> gets a reference (Option<&i32>)
    // .copied -> mutates reference to value (Option<i32>)
    // .unwrap_or -> makes the option the value or 0 if empty
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score of {team_name} is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // always remember that the hashmap owns the values inserted 
    // in it. If we insert references, the values won't be moved
    // in the hashmap, so the reference point must be valid.

    // entry checks if something is present or not
    // insert_or adds a value if false 
    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Purple")).or_insert(10);
    println!("{:?}", scores);

    let text = "anvedi come anvedi balla balla nando";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
