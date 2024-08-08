use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // Accept user input
    // parse user input
    // add Sally to Engineering
    // make everything lowercase
    //
    // check sales -> retrieve list of all people in a department
    // OUTPUT: 
    //  SALES
    //      Sally
    //      James
    //      TuaMadre
    //
    // check -> retrive all people in the company by department
    //  SALES 
    //      ...
    //
    //  ENGINEERING
    //      ...
    //
    //  Use an hash map of vectors
    // if dep doent exist 

    let mut dep_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("dep> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let input = line.trim().to_lowercase();

        let cmd: Vec<&str> = input.split_whitespace().collect();
        if cmd[0] == "add" {
            if cmd.len() < 2 {
                println!("dep> The add command needs at least 2 args");
            }
            let name = String::from(cmd[1]);
            let dep = String::from(cmd[2]);

            dep_map.entry(dep).or_insert_with(Vec::new).push(name);
        }
        if cmd[0] == "show" {
            if cmd.len() == 1 {
                for (key, value) in &dep_map {
                    println!("{}, {:?}", key, value);
                }
            } else {
                let dep = String::from(cmd[1]);
                let people = dep_map.get(&dep);
                match people {
                    Some(vec) => {
                        println!("{:?}", vec);
                    }
                    None => {
                        println!("Nothing found");
                    }
                }
            }
        }


        if cmd[0] == "exit" {
            println!("dep> bye bye");
            break;
        }
        if cmd[0] == "help" {
            println!("dep> help yourself");
        }
    }

    //println!("{}", dep_map.get("some").copied().unwrap_or(0));
}
