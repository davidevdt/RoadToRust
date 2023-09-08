fn main() {
    
    // Unwinding the stack or Aborting in response to a Panic
    {
        // Cargo.toml -> to abort or panic in release mode: 
        // [profile.release]
        // panic = 'abort' 
    }

    {
        // Calling panic 
        // panic!("crash and burn"); 
    }

    {
        // Panic backtrace 
        // let v = vec![1, 2, 3]; 
        // v[99]; 

        // In terminal:
        // $export RUST_BACKTRACE=1  // or any number different from 0
        // cargo run 
    }

    // Recoverable errors with Result 
    
    {
        // The result enum 
        // enum Result<T, E> {
        //     Ok(T), 
        //     Err(E), 
        // }   
    }

    {
        // Trying to open unexisting file 
        use std::fs::File; 
        let greeting_file_result = File::open("hello.txt"); // -> returns a Result<T,E>
                                                            // with T = std::fs::File, E = std::io::Error

        // Handling the result
        let greeting_file = match greeting_file_result {
            Ok(file) => file, 
            Err(error) => panic!("Problem opening the file"), 
        };
    }

    {
        // Matching on different errors 
        use std::fs::File; 
        use std::io::ErrorKind; 

        let greeting_file_result = File::open("hello.txt"); 
        let greeting_file = match greeting_file_result {
            Ok(file) => file, 
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {    // -> because File::create() can also fail 
                    Ok(fc) => fc, 
                    Err(e) => panic!("Problem creating the file: {:?}", e), 
                }, 
                other_error => {
                    panic!("Problem opening the file {:?}", other_error); 
                }
            } 
        }; 
    }

    {
        // Alternative with closures 
        use std::fs::File; 
        use std::io::ErrorKind; 

        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error)
                })
            } else {
                panic!("Problem opening the file {:?}", error); 
            }
        }); 
    }

    {
        // Shortcuts for Panic on Error: unwrap and expect 
        // use std::fs::File 

        // Unwrap: if Result is Ok, returns value inside Ok; if Err, unwrap calls the panic! macro 
        // let greeting_file = File::open("hello.txt").unwrap(); 

        // Expect: like unwrap, but lets you choose error message 
        // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project"); 

    }

    {
        // Propagating errors 
        // -> returns the error to the calling cose rather than handling the error straight away
        use std::fs::File; 
        use std::io::{self, Read}; 

        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("hello.txt"); 

            let mut username_file = match username_file_result {
                Ok(file) => file, 
                Err(e) => return Err(e),
            };

            let mut username = String::new(); 

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username), 
                Err(e) => Err(e), 
            }
        }
    }

    {
        // Shortcut for propagating errors: the ? Operator 
        use std::fs::File; 
        use std::io::{self, Read}; 

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username_file = File::open("hello.txt")?; 
            let mut username = String::new(); 
            username_file.read_to_string(&mut username)?; 
            Ok(username) 
        }

        // Here the ? operator works exactly as match in the previous example 
        // The ? operator calls the from function from the From trait of the std library 
        // to convert error types into the desired one 
        // (with custom error type MyError, we need to implement impl From<io::error> for MyError)
    }

    {
        // Even shorter call with the ? operator 
        use std::fs::File; 
        use std::io::{self, Read}; 

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username = String::new(); 
            File::open("hello.txt")?.read_to_string(&mut username)?; 
            Ok(username)
        }

        // And even shorter with read_to_string: 
        // use std::fs; 
        // use std::io; 
        // fn read_username_from_file() -> Result<String, io::Error> {
            // fs::read_to_string("hello.txt")
        // }
    }

    {
        // When to use the ? operator 
        // In a function that returns Result, Option, or any type that implements FromResidual 
        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last() // if next()'s value is None, None will be returned by last_char_of_first_line
        }

        // To convert Result to Option or vice versa: ok method on Result or ok_or method on Option 

        // main with return: 
        // use std::error::Error; 
        // use std::fs::file; 

        // fn main() -> Result<(), Box<dyn Error>> {
        //     let greeting_file = File::open("hello.txt"); 

        //     Ok(())
        // }

        // The main function can return any type that implements the std::process::Termination trait 
    }

    // panic! or not panic!
    {
        // From the guessing_game program: 
        // validate user input only if it's > 0 and <= 100, Approach #1
        // loop {
        //     // --snip--
        //     let guess: i32 = match guess.trim().parse() {
        //         Ok(num) => num, 
        //         Err(_) => continue, 
        //     }; 
        //     
        //     if guess < 1 || guess > 100 {
        //         println!("The secret number will be between 1 and 100"); 
        //         continue; 
        //     }
        //
        //     match guess.comp(&secret number) {//--snip}    
        // }    

        // Approach 2, with a new type: 
        pub struct Guess {
            value: i32, 
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}.", value); 
                }
                Guess { value }
            }

            pub fn value(&self) -> i32 {
                self.value 
            }
        }
    }


    



}
