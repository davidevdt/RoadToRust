fn main() {
    
    // Unsafe Rust 
    {
        // With a block of code declared unsafe you can: 
        // * dereference a raw pointer 
        // * call an unsafe function or method 
        // * access or modify a mutable static variable 
        // * implement an unsafe trait 
        // * access fields of union S 

        // Dereferencing a raw pointer 
        // Note that we don't need the unsafe keyword: 
        // raw pointers can be created in safe code 
        // Raw pointers can't just be dereferences inside safe code 
        let mut num = 5; 
        let r1 = &num as *const i32;        // -> as: cast a reference into raw pointer 
        let r2 = &mut num as *mut i32;  

        // In this snippet, we're uncertain about the memory location: 
        // let address = 0x012345usize;
        // let r = address as *const i32;

        unsafe {
            println!("r1 is: {}", *r1); 
            println!("r2 is: {}", *r2); 
        }

        // Calling an unsafe function or method 
        unsafe fn dangerous() {}
        unsafe {
            dangerous(); 
        }

        // Creating a safe abstraction over unsafe code 
        let mut v = vec![1, 2, 3, 4, 5, 6]; 
        let r = &mut v[..]; 
        let (a, b) = r.split_at_mut(3); 
        assert_eq!(a, &mut [1, 2, 3]); 
        assert_eq!(b, &mut [4, 5, 6]); 

        // => Rewriting split_at_mut using unsafe code procedures: 
        use std::slice; 
        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len(); 
            let ptr = values.as_mut_ptr(); 

            assert!(mid <= len); 

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid), 
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid), 
                )
            }
        }

        // In contrast, the following snippet is totally unsafe: 
        // we're taking memory at an arbitrary location, and create a 10000-length slice 
        // use std::slice;
        // let address = 0x01234usize;
        // let r = address as *mut i32;
        // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

        // Using extern functions to call external code 
        extern "C" {
            fn abs(input: i32) -> i32; 
        }
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3)); 
        }

        // Digression: Calling Rust functions from other languages 
        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!");
        }

        // Accessing or modifying a mutable static variable 
        // In Rust, global variables are called static, for example: 
        static HELLO_WORLD: &str = "Hello, world!"; 
        println!("name is: {}", HELLO_WORLD); 

        // Access, declare and modify a mutable static variable: 
        static mut COUNTER: u32 = 0; 
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc; 
            }
        }
        add_to_count(3); 
        unsafe {
            println!("COUNTER: {}", COUNTER); 
        }

        // Implementing an unsafe trait 
        unsafe trait Foo {
            // methods go here 
        }

        unsafe impl Foo for i32 {
            // method implementations go here 
        }
    }

    // Advanced traits 
    {
        // Specifying placeholder types in trait definitions with associated types 
        // pub trait Iterator {
        //     type Item;  // -> placeholder, similar to a Generics 
        //     fn next(&mut self) -> Option<Self::Item>; 
        // }

        // impl Iterator for Counter {
        //     type Item = u32; 
        //     fn next(&mut self) -> Option<Self::Item>; 
        // }

        // // while with generics: 
        // pub trait Iterator<T> {
        //     fn next(&mut self) -> Option<T>; 
        // }

        // Default generic type parameters and operator overloading 
        use std::ops::Add; 

        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32, 
            y: i32, 
        }

        impl Add for Point {
            type Output = Point; 

            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x, 
                    y: self.y + other.y,  
                }
            }
        }

        assert_eq!(
            Point{x: 1, y: 0} + Point{x: 2, y: 3}, 
            Point{x: 3, y: 3}, 
        ); 

        // The default generic type in this code is within the Add trait.
        // Rhs=Self is the default type parameter 
        // trait Add<Rhs=Self> {
        //     type Output;

        //     fn add(self, rhs: Rhs) -> Self::Output;
        // }

        // Let's now re-define Add by customizing the Rhs type: 
        struct Millimiters(u32); 
        struct Meters(u32); 

        impl Add<Meters> for Millimiters {
            type Output = Millimiters; 
            fn add(self, other: Meters) -> Millimiters {
                Millimiters(self.0 + (other.0 * 1000)) 
            }
        }

        // Calling methods with the same name
        trait Pilot {
            fn fly(&self); 
        }

        trait Wizard {
            fn fly(&self); 
        }

        struct Human; 

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking"); 
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!"); 
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human; 
        Pilot::fly(&person); 
        Wizard::fly(&person); 
        person.fly(); 

        // Methods without self parameters: 
        trait Animal {
            fn baby_name() -> String; 
        }

        struct Dog; 

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        // println!("A baby dog is called a {}", Dog::baby_name()); // wrong 
        // println!("A baby dog is called a {}", Animal::baby_name()); // compile error -> Rust doesn't know which one to call
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());  

        // So, a fully qualified syntax is defined as: 
        // <Type as Trait>::function(receiver_if_method, next_arg, ...);

        // Using supertraits to require one trait's functionality within another trait 
        use std::fmt; 
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string(); // we can use to_string because we're requiring fmt::Display
                let len = output.len(); 
                println!("{}", "*".repeat(len + 4)); 
                println!("*{}*", " ".repeat(len + 2)); 
                println!("* {} *", output); 
                println!("*{}*", " ".repeat(len + 2)); 
                println!("{}", "*".repeat(len + 4)); 
            }
        }

        struct PointFmt {
            x: i32, 
            y: i32, 
        }

        impl fmt::Display for PointFmt {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        impl OutlinePrint for PointFmt {}


        // Using the NewType pattern to implement Extrnal traits on external types 
        struct Wrapper(Vec<String>); 
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]); 
        println!("w = {}", w); 
    }

    // Advanced Types 
    {
        // Type synonims with Type Aliases 
        type Kilometers = i32; 
        let x: i32 = 5; 
        let y: Kilometers = 5; 
        println!("x + y = {}", x + y); 

        // ..useful for long names: 
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi"));
        // fn takes_long_type(f: Thunk) {
        //     // --snip--
        // }
        // fn returns_long_type() -> Thunk {
        //     // --snip--
        // }

        // Type aliases are commonly used with Resul<T, E> type 
        use std::fmt; 
        use std::io::Error; 
        // pub trait Write {
        //     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>; 
        //     fn flush(&mut self) -> Result<(), Error>; 

        //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>; 
        //     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>; 
        // }

        // This can become: 
        type Result<T> = std::result::Result<T, std::io::Error>; 
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>; 
            fn flush(&mut self) -> Result<()>; 

            fn write_all(&mut self, buf: &[u8]) -> Result<()>; 
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>; 
        }


        // The never type that never returns 
        // A function that returns never (diverging function)
        // fn bar() -> ! {
        //     // -- snip -- 
        // }

        // From the Guess Game snippet: 
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num, 
        //     Err(_) => continue, 
        // }; 

        // Match arms must always return the same type, so this won't work: 
        // let guess = match guess.trim().parse() {
        //     Ok(_) => 5,
        //     Err(_) => "hello",
        // };

        // 'continue' in the previous snippet works because it has a ! value 
        // Because ! never has a value, Rust decides the type of guess is u32 
        // In practice ! can be coerced to any other type 

        // panic! and "loop" (called without using break) also are ! type 


        // Dynamically sized types (DST) and the Sized trait
        // The following won't compile as Rust doesn't know the size of a str
        // But using &str instead of str it would work 
        // let s1: str = "Hello there!"; 
        // let s2: str = "How's it going?"; 

        // The Sized trait allows to work with DST's to determine if a type's size
        // is known at compile time 
        fn generic<T>(t:T) {
            // -- snip -- 
        }
        // this is implicitly converted into this at compile time: 
        fn generic_two<T: Sized>(t: T) {
            // -- snip --
        }
        // To relax working with types that have a known size at compile time: 
        // (?Sized = “T may or may not be Sized”)
        fn generic_three<T: ?Sized>(t: &T) {
            // -- snip -- 
        }
    }

    // Advanced functions and closures 
    {
        // Funciton pointers 
        fn add_one(x: i32) -> i32 {
            x + 1 
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5); 
        println!("The answer is: {}", answer); 

        // Example for either closure or named function: 
        let list_of_numbers = vec![1, 2, 3]; 
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect(); 
        // Or alternatively:
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect(); 

        // Enum variant as initializer functions 
        enum Status {
            Value(u32), 
            Stop, 
        } 

        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect(); 
        
        
        // Returning closures 
        fn return_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }

    // Macros 
    {   
        // Macros are known as metaprogramming 
        // declarative macros: macro_rules!
        // Procedural macros: 
        // *    #[derive]
        // *    attribute-like macros 
        // *    function-like macros 

        // macros must be brought to scope before they're called in a file, 
        // as opposed to functions that can be defined and called anywhere 

        // Declarative macros 
        // -- The vec! macro -- 

        // #[macro_export]
        // macro_rules! vec {
        //     ( $( $x: expr), * ) => {                // similar to match expression 
        //         let mut temp_vec = Vec::new(); 
        //         $(
        //             temp_vec.push($x); 
        //         )* 
        //         temp_vec
        //     }
        // }; 

        // $ declares variable in the macro system (denotes a macro variable)
        // $() parenthesis match the patter within parenthesis 
        // $x: expr matches any rust expression and gives it the name x 
        // , after $() indicates that an optional comma separator might appear after what is in $() 
        // * specifies that the pattern matches zero or more of whatever comes before the * 
        // temp_vec.push() within $()* is generated for each part that matches $()
        // So if we call vec![1, 2, 3] this will be generated:  
        // {
        //     let mut temp_vec = Vec::new(); 
        //     temp_vec.push(1); 
        //     temp_vec.push(2); 
        //     temp_vec.push(3); 
        //     temp_vec 
        // }


        // Procedural macros for generating code from attributes 
        // use proc_macro; 

        // #[some_attribute]
        // pub fn some_name(input: TokenStream) -> TokenStream {
        // }


        // Write a custom defined macro 
        
        // $ cargo new hello_macro --lib
        // Filename: src/lib.rs
        pub trait HelloMacro {
            fn hello_macro();
        }

        // $ cargo new hello_macro_derive --lib
        // file: hello_macro_derive/Cargo.toml
        // [lib]
        // prob-macro = true 
        // [dependencies]
        // syn = "1.0"
        // quote = "1.0"

        // filename: hello_macro_derive/src/lib.rs
        // use proc_macro::TokenStream;
        // use quote::quote;
        // use syn;

        // #[proc_macro_derive(HelloMacro)]
        // pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
        //     // Construct a representation of Rust code as a syntax tree
        //     // that we can manipulate
        //     let ast = syn::parse(input).unwrap();

        //     // Build the trait implementation
        //     impl_hello_macro(&ast)
        // }

        // 
        //
        // fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
        //     let name = &ast.ident;
        //     let gen = quote! {
        //         impl HelloMacro for #name {
        //             fn hello_macro() {
        //                 println!("Hello, Macro! My name is {}!", stringify!(#name));
        //             }
        //         }
        //     };
        //     gen.into()
        // }

        // Now in the main package: 
        // add this to dependencies:
        // hello_macro = { path = "../hello_macro" }
        // hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }

        // use hello_macro::HelloMacro; 
        // use hello_macro_derive::HelloMacro; 

        // #[derive(HelloMacro)]
        struct Pancakes; 

        impl HelloMacro for Pancakes {
            fn hello_macro() {
                println!("Hello, Macro! My name is Pancakes!");
            }
        }
        
        Pancakes::hello_macro(); 


        // Attribute-like macros 
        // #[route(GET, "/")]
        // fn index() {...}

        // #[proc_macro_attribute]
        // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

        // Function-like macros 
        // let sql = sql!(SELECT * FROM posts WHERE id=1);

        // #[proc_macro]
        // pub fn sql(input: TokenStream) -> TokenStream {

    }
}
