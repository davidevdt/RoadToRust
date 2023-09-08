fn main() {
    /*

    # Default optimization levels:

    Cargo.toml: 

    [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 3

    # Documentation comments: 

    /// Adds one to the number given. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// let arg = 5; 
    /// let answer = my_crate::add_one(arg); 
    /// 
    /// assert_eq!(6, answer); 
    /// ``` 
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }

    To generate the documentation: 
    $ cargo doc 
    To open the documentation:
    $ cargo doc --open 

    # Commonly used sections: 
    * Panics 
    * Errors 
    * Safety 

    # Running documentation comments as tests 
    $ cargo test 
    will run the exmaples in the documentation, to see if they work  

    # Documenting the crate/file containing the comments 
    Example: 

    //! # My Crate
    //! `my_crate` is a collection of utilities to make performing ceraint
    //! calculations more convenient. 

    // Adds on to the given number. 
    // --snip--

    # Exporting a convenient public API with pub use 

    Suppose you write a art library as in the following example: 

    filename: src/lib.rs 
    //! # Art 
    //! 
    //! A library for modeling artistic concepts. 
    
    pub mod kinds {
        /// The primary colors according to the RYB model.
        pun enum PrimaryColor {
            Red, 
            Yellow, 
            Blue, 
        }

        /// The secondary colors according to the RYB color model. 
        pub enum SecondaryColor {
            Orange, 
            Green, 
            Purple,
        }
    }

    pub mod utils {
        use crate::kinds::*; 

        /// Combines two priamry colors in equal amoutns to create 
        /// a secondary color. 
        pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
            // --snip--
        }   
    }

    Now suppose someone is using this library, he needs to import what he needs: 

    filename: src/main.rs 
    use art::kinds::PrimaryColor; 
    use art::utils::mix; 

    fn main() {
        let red = PrimaryColor::Red; 
        let yellow = PrimaryColor::Yellow;
        mix(red, yellow);  
    }


    It is possible to simplify developers' life by specifying this in our crate: 
    filename: src/lib.rs 
    //! # Art
    //!
    //! A library for modeling artistic concepts.

    pub use self::kinds::PrimaryColor;
    pub use self::kinds::SecondaryColor;
    pub use self::utils::mix;

    pub mod kinds {
        // --snip--
    }

    pub mod utils {
        // --snip--
    }

    And now the library can be used as follows: 

    filename: src/main.rs 
    use art::PrimaryColor; 
    use art::mix; 

    fn main() {
        // -- snip --
    }

    # Setting up cargo account 
    $ cargo login api_key  // -> API keys retrieved from  https://crates.io/me/ 

    # Adding metadata to a new crate 

    filename: Cargo.toml 
    [package]
    name = "guessing_game"
    version = "0.1.0"
    edition = "2021"
    description = "A fun game where you guess what number the computer has chosen."
    license = "MIT OR Apache-2.0"

    [dependencies]

    # Publishing to crates.io
    $ cargo.publish 

    # Deprecating a version with cargo yank 
    $ cargo yank --vers 1.0.1

    And to undo a yank: 
    $ cargo yank --vers 1.0.1 --undo 

    # Cargo workspaces 
    -> Workspace: set of package under the same Cargo.lock 

    $ mkdir add 
    $ cd add 

    filename: Cargo.toml 
    [workspace]

    members = [
        "adder", 
    ]

    $ cargo new adder       // -> inside add directory 

    -> Creating the second package in the workspace: 
    [workspace]

    members = [
        "adder", 
        "add_one", 
    ]

    $ cargo new add_one --lib 

    filename: add_one/src/lib.rs 
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }

    filename: adder/Cargo.toml 
    [dependencies]
    add_one = {path= "../add_one"}

    filename: adder/src/main.rs 
    use add_one; 

    fn main() {
        let num = 10; 
        prinln!("Hello, world! {num} plus one is {}", add_one::add_one(num)); 
    }

    -> From the top-level add directory: 
    $ cargo build 

    -> To run binary crate from add directory we can specify which package to use: 
    $ cargo run -p adder 

    -> Depending on extrnal packages in a workspace: 

    filename: add_one/Cargo.toml
    [dependencies]
    rand = "0.8.5"

    filename: adder/Cargo.toml
    [dependencies]
    rand = "0.8.5"

    -> Adding test to workspace:
    filename: add_one/src/lib.rs 
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            assert_eq!(3, add_one(2));
        }
    }

    From top-level add directory: 
    $ cargo test 

    To run only add_one test: 
    cargo test -p add_one 

    # Installing binaries with cargo install 
    Binaries installed with cargo install are stored inside root's bin folder. 
    By default, this is in the $PATH and it is: 
    $ HOME/.cargo/bin

     */
}
