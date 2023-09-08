fn main() {

    // Ownership 

    {
        // Variable scope 

        let s = "hello"; // -> string literal 
        // do stuff with s 
    } // -> out of scope: s no longer valig 

    {
        // String data type -> allocated on the heap 
        // let s = String::from("hello"); // -> immutable 
        let mut s = String::from("hello"); 
        s.push_str(", world!"); // -> appends a literal to a string 
        println!("{}", s);
    }

    // Memory and allocation 
    {
        // Automatic memory deallocation when memory owner goes out of scope
        // {
        //    let s = String::from("hello"); // -> s is valid from this point forward 
        // ... do stuff with s 
        // }   // -> the scope is now over, s is no longer valid 
        // (behind the scenes, Rust calls the drop function)

    }

    // Copying 
    {
        // On the stack 
        let x = 5; 
        let y = x; // -> Values of known size, copied on the stack. A normal copy, as expected, is performed 
        println!("x = {}, y = {}", x, y);
    }

    {
        // On the heap 
        // No copy, but moving by default 
        // let s1 = String::from("hello"); 
        // let s2 = s1; 
        // println!("{}, world!", s1);      // -> Compile Error. s1 is MOVED INTO s2, rather than copied. 
                                            // This means s1 is invalid at this point. 
                                            // When s2 goes out of scope it will be the only object freeing memory 
                                            // This prevents conflicts in the potentially shared memory between s1 and s2 

        // Deep copy 
        let s1 = String::from("hello"); 
        let s2 = s1.clone(); 
        println!("s1 = {}, s2 = {}", s1, s2); // -> This will work, as we performed a deep copy
        
    }

    {
        // Copy on stack = implement Copy traits
        let x = 5; 
        let y = x; 
        println!("x = {x}, y = {y}"); 

        // Types that implement Copy (as types in the stacks): 
        // -> all integers (i.e. i32, u64, ...)
        // -> booleans 
        // -> all floating-point numbers 
        // -> characters
        // -> tuples (if only contain Copy-trait types: i.e. (i32, i32) YES; (i32, String) DOES NOT); 
        // * If a type implements Drop, it won't be allowed to implement Copy 
    }

    // Ownership and functions 
    {
        // Taking ownership and taking copies 

        fn takes_ownership(some_string: String) {   // some_strings comes into scope 
            println!("{}", some_string);              
        }   // some_string goes out of scope and 'drop' is called. The backing memory is freed 

        fn makes_copy(some_integer: i32) {      // some_integer comes into scope 
            println!("{}", some_integer); 
        }   // some_integer goes out of scope. Nothing special happens. 

        let s = String::from("hello"); // -> s comes in scope 
        takes_ownership(s);            // -> s's value goes into the function, so no longer valid in this scope 
        // takes_ownership(s.clone());            // -> Here s is copied, so it will be valid afterwards; 
        // println!("In scope, s = {}", s); // -> Compile error if s is not cloned into the function 

        let x = 5;                     // -> x comes in scope 
        makes_copy(x);                 // -> x would move into the function, but i32 is Copy type, so x can be use afterwards
        println!("In scope, x = {}", x); // -> This will work 
    } // -> Here x goes out of scopes. Then s does. But because s's value was moved, nothing special happens. 

    
    {
        // Returning ownership  

        fn gives_ownership() -> String {    // -> It will move its return values into the scope calling it 
            let some_string = String::from("yours"); // -> string comes into scope 
            return some_string;   // -> returned and moves out to the calling function 
        }

        fn takes_and_gives_back(a_string: String) -> String { // -> a_string comes into scope 
            return a_string; // -> a_string is returned and moves out to the calling function 
        }

        let s1 = gives_ownership();         // -> the return value of gives_ownership is moved into s1 
        let s2 = String::from("hello");     // -> s2 comes into scope 
        let s3 = takes_and_gives_back(s2); // -> s2 moved into takes_and_gives_back; in turn, the function returns the value to s3 
    } // -> Out of scope: s1, s3 will be dropped. 

    // !!! The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it
    // !!! When a variable that includes data on the heap goes out of scope, 
    // !!! the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

    
    {
        // Returning multiple values through tuple 
        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); 
            (s, length) 
        }

        let s1 = String::from("hello"); 
        let (s2, len) = calculate_length(s1); 
        println!("The length of '{}' is {}.", s2, len); 
    }

    // References and borrowing
    {   
        // Function with reference 

        fn calculate_length(s: &String) -> usize {    // the string is borrowed 
            s.len()
        }   // s goes out of scope, but because it does not own the string, it is not dropped 

        let s1 = String::from("hello"); 
        let len = calculate_length(&s1); 
        println!("The length of {} is {}.", s1, len); 
    }

    
    {
        // Changing borrowed values 

        fn change(some_string: &mut String) {   // -> Without mut we could not change it 
            some_string.push_str(", world"); 
        }
        
        // let s = String::from("hello"); // -> if we gave this as an input, we could not pass it to the function
        let mut s = String::from("hello"); // -> With mut we can not change it 
        change(&mut s);     // -> Without mut we could not change it 
        println!("Changed string: {}", s); 
    }

    
    {   
        // Mutable references cannot be borrowed more than once
        // The following snippet won't work: 
        // let mut s = String::from("hello"); 
        // let r1 = &mut s; 
        // let r2 = &mut s; 
        // println!("{}, {}", r1, r2); // -> It won't work 
        // Benefit: it prevents data races 
    }

    
    {   
        // New scopes can allow for mutable references 
        let mut s = String::from("hello");
        {
            let r1 = &mut s; 
        }   // r1 goes out of scope here, it is dropped; 
        let r2 = &mut s; // -> this is now valid
    }

    {
        // At any time, you can have either one mutable reference or a number of immutable references 
        // (specifically, a reference to a mutable type)
        // Multiple mutable and immutable references cannot be combined either 
        let mut s = String::from("hello"); 
        let r1 = &s; // -> no problem 
        let r2 = &s; // -> no problem 
        let r3 = &mut s; // -> problem: not allowed 
    } 
    
    {
        // The reference scope starts when it is introduce and ends when it is not used anymore 
        let mut s = String::from("hello");
        let r1 = &s; 
        let r2 = &s; 
        println!("{} and {}", r1, r2); 
        // r1 and r2 not used anymore after this point 

        let r3 = &mut s; // -> no problem now: scopes of the mutable and immutable refs do not overlap 
        println!("{}", r3); 
    }

    {   
        // Dangling references 
        // fn dangle() -> &String { // dangle returns a reference to a String
        //     let s = String::from("hello"); // s is a new string 
        //     &s // we return a reference to the string, s 
        // } // s goes out of scope and is dropped -> the reference to the data is not valid anymore; 
        //   // compiler will avoid dangling pointer with an error
        
        // let reference_to_nothing = dangle(); // -> see dangle() function for explanation 

        // Solution: return the String directly
        fn no_dangle() -> String {
            let s = String::from("hello"); 
            s 
        }   
        let s = no_dangle(); 
    }

    // The Slice type 
    // Slices: references to a contiguous sequence of elements in a collection 
    {
        // First word with references: 
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes(); 

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i; 
                }
            }
            s.len() 
        }

        let mut s = String::from("hello world"); 
        let word = first_word(&s); // word will get the value 5 
        s.clear(); // this empties the String, which is now ""
        // word still have the value 5 here, but there's no more string 
        // that we could meaningfully use the value 5 with. word is then invalid 
        // However, unfortunately we won't get compile error here, but a runtime bug 
    }
    
    {
        // String slices 
        let s = String::from("hello world"); 
        let hello = &s[0..5];       // -> slice ref of length: end_index - start_index 
        let world = &s[6..11];      // -> slice ref of length: end_index - start_index 

        // The following are equivalent: 
        let s = String::from("hello"); 
        let slice = &s[0..2]; 
        let slice = &s[..2]; 

        // The following are equivalent: 
        let len = s.len(); 
        let slice = &s[3..len]; 
        let slice = &s[3..]; 

        // The following are equivalent: 
        let len = s.len(); 
        let slice = &s[0..len]; 
        let slice = &s[..]; 
        let slice = &s; 
    }

    {
        // First word with slices (&String as argument): 
        fn first_word(s: &String) -> &str {     // &str is a string slice 
            let bytes = s.as_bytes(); 
        
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i]; 
                }
            }
            &s[..]
        }

        let mut s = String::from("hello world"); 
        let word = first_word(&s); 
        s.clear();   
        // println!("the first word is: {}", word); // Compile time error here (mutable and immutable ref 
                                                   // in the same scope through calls to .clear and println! but this cannot happen)
    }
    
    {
        // String literals are immutable => &str are immutable references
        let s = "Hello world"; // -> s is &str 
    }

    {
        // First word with string slices as argument 
        // (string slices as arguments allow for both string slices &str and String references &String as inputs)

        fn first_word(s: &str) -> &str {          // now also string slices and string literals can be used as arguments 
            let bytes = s.as_bytes(); 
        
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i]; 
                }
            }
            &s[..]
        }

        let my_string = String::from("hello world"); 
        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word(&my_string[0..6]); 
        let word = first_word(&my_string[..]); 
        // `first_word` also works on references to `String`s
        let word = first_word(&my_string); // -> equivalent to first_word(&my_string[..]) 

        // The same can be done for string literals: 
        let my_string_literal = "hello world"; 
        let word = first_word(&my_string_literal[0..6]); 
        let word = first_word(&my_string_literal[..]); 
        let word = first_word(my_string_literal); // because string literals are already &str 
    }

    {
        // Other slices 
        let a = [1, 2, 3, 4, 5]; 
        let slice = &a[1..3]; // array slice, of type &[i32] 
        assert_eq!(slice, &[2, 3]); 
    }

}




