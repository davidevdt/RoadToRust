fn main() {
    
    // Generic Data types 
    {
        // Two functions that calculate largest i32 and largest char 
        fn largest_i32(list: &[i32]) -> &i32 {
            let mut largest = &list[0]; 

            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        fn largest_char(list: &[char]) -> &char {
            let mut largest = &list[0]; 
            for item in list {
                if item > largest {
                    largest = item; 
                }
            }
            largest 
        }

        let number_list = vec![34, 50, 25, 100, 65]; 
        let result = largest_i32(&number_list); 
        println!("The largest number is {}", result); 

        let char_list = vec!['y','m','a','q']; 
        let result = largest_char(&char_list);
        println!("The largest char is {}", result); 
    }  

    {
        // Calculating largest with generics [won't compile for the moment]
        // fn largest<T>(list: &[T]) -> &T {
        //     let mut largest = &list[0]; 
        //     for item in list {
        //         if item > largest {
        //             largest = item; 
        //         }
        //     }
        //     largest
        // }

        // let number_list = vec![34, 50, 25, 100, 65]; 
        // let result = largest(&number_list); 
        // println!("The largest number is {}", result); 

        // let char_list = vec!['y','m','a','q']; 
        // let result = largest(&char_list);
        // println!("The largest char is {}", result); 
    }

    {
        // In struct definitions 
        struct Point<T> {
            x: T, 
            y: T, 
        }

        let integer = Point {x: 5, y: 10};  
        let float = Point {x: 1.0, y: 4.0}; 
        // let wont_work = Point { x: 5, y: 4.0 }; // This won't compile, as x and y have two different types 
    }

    {
        // Using different types 
        struct Point<T, U> {
            x: T, 
            y: U, 
        }
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };

    }

    {
        // In enum definitions 
        // enum Option<T> {
        //     Some(T), 
        //     None, 
        // }

        // enum Result<T, E> {
        //     Ok(T), 
        //     Err(E), 
        // }
    }

    {
        // In method definitions 
        struct Point<T> {
            x: T, 
            y: T, 
        }

        impl<T> Point<T> {      // -> here, the first <T> indicates that we are writing an impl for the generic Point<T>
            fn x(&self) -> &T {
                &self.x 
            }
        }

        let p = Point{x: 5, y: 10}; 
        println!("p.x = {}", p.x()); 

        // Restrict methods to Point<f32>: 
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
    }

    {
        // Different types between struct and its methods 
        struct Point<X1, Y1> {
            x: X1, 
            y: Y1, 
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x, 
                    y: other.y, 
                }
            }
        }

        let p1 = Point{ x: 5, y: 10.4}; 
        let p2 = Point{ x: "Hello", y: 'c'}; 
        let p3 = p1.mixup(p2); 
        println!("p3.x = {}, p3.y  = {}", p3.x, p3.y); 
    }

    {
        // Performance of code using generics 
        let integer = Some(5); 
        let float = Some(5.0);  

        // Monomorphization of the code makes the previous lines of code work as follows behind the scenes: 
        enum Option_i32 {
            Some(i32), 
            None, 
        }

        enum Option_f64 {
            Some(f64), 
            None, 
        }

        let integer = Option_i32::Some(5); 
        let float = Option_f64::Some(5.0); 
    }

    // Traits [Defining shared behaviour]
    {
        // Implementing a trait for a NewsArticle type 
        pub trait Summary {               // A trait is like an interface  
            fn summarize(&self) -> String; 
        }

        pub struct NewsArticle {
            pub headline: String, 
            pub location: String, 
            pub author: String, 
            pub content: String, 
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String, 
            pub content: String, 
            pub reply: bool, 
            pub retweet: bool, 
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        // use aggregator::{Summary, Tweet}; 
        let tweet = Tweet {
            username: String::from("horse_ebooks"), 
            content: String::from("of course, as you probably already know, people"), 
            reply: false, 
            retweet: false, 
        }; 
        println!("1 new tweet: {}", tweet.summarize()); 
    }

    {
        // Default implementations 

        pub struct NewsArticle {
            pub headline: String, 
            pub location: String, 
            pub author: String, 
            pub content: String, 
        }

        // Summary trait with default implementaiton for summarize 
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }

        impl Summary for NewsArticle{}

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup championship!"), 
            location: String::from("Pittsburgh, PA, USA"), 
            author: String::from("Iceburgh"), 
            content: String::from(
                "The Pittsburgh Penguins once again are the best\
                hockey team in the NHL.")
        };

        println!("New article available! {}", article.summarize()); 
    }

    {
        // Default implementations can call methods in the same trait without default implementation 
        pub struct Tweet {
            pub username: String, 
            pub content: String, 
            pub reply: bool, 
            pub retweet: bool, 
        }

        pub trait Summary {
            
            fn summarize_author(&self) -> String; 

            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"), 
            content: String::from("of course, as you probably already know, people"), 
            reply: false, 
            retweet: false, 
        }; 

        println!("1 new tweet: {}", tweet.summarize()); 
    }

    {
        // Using traits as parameters 
        // pub fn notify(item: &impl Summary) {    // -> it accepts both Tweet and NewsArticle types
        //    println!("Breaking news! {}", item.summarize()); 
        // }
    }

    {
        // Trait bound syntax 
        // pub fn notify<T: Summary>(item: &T) {
        //     println!("Breaking news! {}", item.summarize()); 
        // }

        // With two parameters: 
        // pub fn notify(item1: &impl Summary, item2: &impl Summary) {
        // pub fn notify<T: Summary>(item1: &T, item2: &T) {
    }

    {
        // Specify multipe trait bounds 
        // pub fn notify(item: &(impl Summary + Display)) {
        // pub fn notify<T: Summary + Display>(item: &T) {
    }

    {
        // Where Clauses 
        // Instead of this: 
        // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        // ... we can write this: 
        // fn some_function<T, U>(t: &T, u: &U) -> i32 
        // where 
        //     T: Display + Clone, 
        //     U: Clone + Debug, 
        // {
    }

    {
        // Returning Types that implement traits 
        // fn returns_summarizable() -> impl Summary {
        //     Tweet {
        //         username: String::from("horse_ebooks"),
        //         content: String::from(
        //             "of course, as you probably already know, people",
        //         ),
        //         reply: false,
        //         retweet: false,
        //     }
        // }

        // However this is not possible: only one Trait type can be returned 
        // fn returns_summarizable(switch: bool) -> impl Summary {
        //     if switch {
        //         NewsArticle {
        //             headline: String::from(
        //                 "Penguins win the Stanley Cup Championship!",
        //             ),
        //             location: String::from("Pittsburgh, PA, USA"),
        //             author: String::from("Iceburgh"),
        //             content: String::from(
        //                 "The Pittsburgh Penguins once again are the best \
        //                  hockey team in the NHL.",
        //             ),
        //         }
        //     } else {
        //         Tweet {
        //             username: String::from("horse_ebooks"),
        //             content: String::from(
        //                 "of course, as you probably already know, people",
        //             ),
        //             reply: false,
        //             retweet: false,
        //         }
        //     }
        // }
    }

    {
        // Conditionally implement methods with trait bounds 
        use std::fmt::Display; 

        struct Pair<T> {
            x: T, 
            y: T, 
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self{ x, y }
            }
        }

        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x > self.y {
                    println!("The largest member is x = {}", self.x); 
                } else {
                    println!("The largest member is y = {}", self.y); 
                }
            }
        }

        // Blanket implementations (conditional implement a trait for any type that implements another trait)
        // impl<T: Display> ToString for T {...}
    }

    // Lifetimes 
    {
    //     // Preventing dangling references with lifetimes 
    //     let r = 10; // note: Rust allows declaring variables without value, but if used before giving it a value, we'll get compile-time error
    //     {
    //         let x = 5; 
    //         r = &x; 
    //     }
    //     println!("r: {}", r)    // -> Value of r gone out of scope before we used it, compile error
    //
    //      Rust uses a borrow checker to determine whether borrows are vaid. 
            // This code will work: 
            let x = 5; 
            let r = &x; 
            println!("r: {}", r); 
    }

    {
        // Generic lifetimes in functions 
        // This won't work: as the compiler doesn't know the lifetime of x and y 
        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }

        // let string1 = String::from("abcd"); 
        // let string2 = "xyz"; 
        // let result = longest(string1.as_str(), string2); 
        // println!("The longest string is {}", result); 

        // &i32 -> a reference 
        // &'a i32 -> a reference with explicit lifetime 
        // &'a mut i32 -> a mutable reference with an explicit lifetime 

        // What we are saying now is: the returned reference will be valid as long as both parameters will be valid 
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        // From the book: 
        // The function signature now tells Rust that for some lifetime 'a, 
        // the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. 
        // The function signature also tells Rust that the string slice returned from the function will live at least as long as 
        // lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same 
        // as the smaller of the lifetimes of the values referred to by the function arguments. 

        // Here: string 1 is valid until the end of the outer scope; string2 and result valid until end of inner scope 
        let string1 = String::from("long string is long");
        { 
            let string2 = String::from("xyz"); 
            let result = longest(string1.as_str(), string2.as_str()); 
            println!("The longest string is {}", result); 
        }

        // Here: compile error, as result is valid only the end of the inner scope (otherwise string2 should be in outer scope too)
        let string1 = String::from("long string is long");
        let result; 
        { 
            let string2 = String::from("xyz"); 
            result = longest(string1.as_str(), string2.as_str()); 

            // However this does work: 
            // let string2 = "xyz"; 
            // result = longest(string1.as_str(), string2); 
        }
        println!("The longest string is {}", result); 

    }

    {
        // Thinking lifetimes 
        // If instead of the longest string we returned only the first string parameter, 
        // we wouldn't need to specify a lifetime on y: 
        // fn longest<'a>(x: &'a str, y: &str) -> &'a str {}

        // If the return reference is created in the function, this won't compile: 
        // fn longest<'a>(x: &str, y: &str) -> &'a str {
        //     let result = String::from("really long string");
        //     result.as_str()
        // }
        // This would create a dangling reference. Here, the best option is to return an owned data type 
    }

    {
        // Lifetime annotations in struct definitions 
        // Structs that hold references (lifetime annotation is required in this case)
        struct  ImportantExcerpt<'a> {
            part: &'a str, 
        }

        let novel = String::from("Call me Ishmael. Some years ago..."); 
        let first_sentence = novel.split('.').next().expect("Could not find a '.'"); 
        let i = ImportantExcerpt {
            part: first_sentence; 
        }
    }

    {
        // Lifetime elision 
        // The following function (signature) does not require lifetime annotations 
        // fn first_word(s: &str) -> &str 
        // due to the three rules of lifetime elision: 
        // i) compiler assigns a lifetime parameter to each parameter that's a reference: 
        // fn foo<'a>(x: &'a i32); fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
        // ii) compiler assigns to all output lifetime parameters the same lifetime of the lifetime parameter of the 
        // function argument, if such argument is alone in the signature: 
        // fn foo<'a>(x: &'a i32) -> &'a i32
        // iii) if, among multiple input lifetime parameters, one of them is &self or &mut self, the lifetime of self 
        // is assigned to all output lifetime parameters. 
        // 
        // Therefore, given the three rules, the original signature: 
        // fn first_word(s: &str) -> &str 
        // is inferred as: 
        // fn first_word<'a>(s: &'a str) -> &str -> by rule i 
        // fn first_word<'a>(s: &'a str) -> &'a str { -> by rule ii 
        // 
        // Let's now try with the longest() function above: 
        // fn longest(x: &str, y: &str) -> &str -> original signature 
        // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str -> rule i 
        // Now, the second rule doesn't apply, because the is more than one lifetime parameter. Rule iii doesn't apply either, 
        // as longest is a function, not a method. 
    }

    {
        // Lifetime annotations in method definitions 
        struct  ImportantExcerpt<'a> {
            part: &'a str, 
        }

        // Elision rule (i) applies here: 
        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
        }

        // Elision rule (iii) applies here: 
        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {    // Elision rules (i for inputs) + (iii for output)
                println!("Attention please: {}", announcement); 
                self.part 
            }
        }
    }

    {
        // The static lifetime 
        // static: the affected reference can live for the entire duration of the program 
        // All string literals have statice lifetime, therefore they can be annotated as follows: 
        let s = &'static str = "I have a static lifetime"; 
    }

    // Generic Type Parameters, Trait Bounds, and Lifetimes together 
    use std::fmt::Display; 

    fn longest_with_an_announcement<'a, T>(
        x: &'a str, 
        y: &'a str, 
        ann: T, 
    ) -> &'a str 
    where   
        T: Display 
    {
        println!("Announcement! {}", ann); 
        if x.len() > y.len() {
            x 
        } else {
            y
        }
    }



}
