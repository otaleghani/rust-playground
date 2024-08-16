use std::collections::HashMap;

fn main() {
    let v = vec![1,2,3,4,5,6,1,3,3];
    let mut map = HashMap::new();
    let mut sum = 0;

    for i in &v {
        sum += i;
        let count = map.entry(i).or_insert(0);
        *count += 1; 
    }
    
    let mut modulo = 0;
    let mut freq = 0;

    for (key, value) in &map {
        if *value > freq {
            modulo = **key;
            freq = *value;
        }
    }

    let median = sum / v.len();
    println!("Sum {sum}");
    println!("Median {median}");
    println!("Modulo {modulo}");
}
