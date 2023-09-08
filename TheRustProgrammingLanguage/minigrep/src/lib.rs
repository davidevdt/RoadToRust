use std::fs; 
use std::error::Error; 
use std::env; 

pub struct Config {
    pub query : String, 
    pub file_path: String, 
    pub ignore_case: bool, 
}

impl Config {
    // fn new(args: &[String]) -> Config {

    //     if args.len() < 3 {
    //         panic!("Not enough arguments");
    //     }

    //     let query = args[1].clone(); 
    //     let file_path = args[2].clone(); 

    //     Config {query, file_path}
    // }

    // pub fn build(args: &[String]) -> Result<Config, &'static str> { // -> changed after chapter 13 
    pub fn build(mut args: impl Iterator<Item = String>, ) -> Result<Config, &'static str> {

        // Body of Chapter 12: 
        // if args.len() < 3 {
        //     return Err("not enough arguments")
        // }

        // let query = args[1].clone(); 
        // let file_path = args[2].clone(); 
        // let ignore_case = env::var("IGNORE_CASE").is_ok(); // we don't care about the value of the variable, only if it exists 
        // Ok(Config {
        //     query, 
        //     file_path, 
        //     ignore_case, 
        // })

        // Body of Chapter 13: 
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

        Ok(Config{
            query, 
            file_path, 
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; //.expect("Unable to read file.");
    // println!("With text:\n{contents}"); 

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
        // Try running with: IGNORE_CASE=1 cargo run -- to poem.txt
        // to unset: unset IGNORE_CASE
    } else {
        search(&config.query, &contents)
    }; 

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Body of Chapter 12: 
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line); 
    //     }
    // }
    // results 

    // Body of Chapter 13: 
    contents 
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
        let query = "duct"; 
        let contents = "\
        Rust:\nsafe, fast, productive.\nPick three.\nDuct tape."; 

        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); 
    }

    #[test]
    fn case_insensitive() {
        let query = "ruST"; 
        let contents = "\
        Rust:\nsafe, fast, productive.\nPick three.\nTrust me."; 

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents)); 
    }
}