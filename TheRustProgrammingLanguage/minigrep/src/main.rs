use std::env; 
use std::process; 

use minigrep::Config; 


fn main() {
    // let args: Vec<String> = env::args().collect(); // -> commented out after chapter 13 
    // let config = Config::new(&args); 

    // let config = Config::build(&args).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| { // -> change after chapter 13 
        eprintln!("Problem parsing arguments: {err}"); // eprintln! prints to stderr // e.g. cargo run 2> output.log or cargo run > output.log 2>&1 
        process::exit(1); 
    }); 

    // println!("Searching for {}", config.query); 
    // println!("In file {}", config.file_path); 

    // let contents = fs::read_to_string(config.file_path).expect("Unable to read file."); 

    // println!("With text:\n{contents}");

    if let Err(e) = minigrep::run(config) {   // no need for unwrap_or_else here as Ok() is () in run()
        eprintln!("Application error: {e}"); 
        process::exit(1); 
    } 

}



