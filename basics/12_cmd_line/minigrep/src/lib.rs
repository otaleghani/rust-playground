use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // OLD way of building the config without an iterator
    // pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("Not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();
    //     Ok(Config { 
    //         query, 
    //         file_path,
    //         ignore_case,
    //     })
    // }
    
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

// OLD search function without iterators
// pub fn search<'a>(
//     query: &str,
//     contents: &'a str,
// ) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }

pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "anvedi";
        let contents = "anvedi come balla nando";
        assert_eq!(
            vec!["anvedi come balla nando"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "anVEDI";
        let contents = "anvedi come balla nando";
        assert_eq!(
            vec!["anvedi come balla nando"],
            search_case_insensitive(query, contents)
        );
    }
}
