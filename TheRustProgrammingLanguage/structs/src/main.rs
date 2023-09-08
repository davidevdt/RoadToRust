fn main() {

    // Structs 
    struct User {
        active: bool, 
        username: String, 
        email: String, 
        sign_in_count: u64
    }

    {
        // Defining and instantiating structs 
        // Immutable 
        let user1 = User {
            active: true, 
            username: String::from("someusername123"), 
            email: String::from("someone@example.com"), 
            sign_in_count: 1, 
        };
    }

    {
            // Mutable struct 
            let mut user1 = User {
                active: true, 
                username: String::from("someusername123"), 
                email: String::from("someone@example.com"), 
                sign_in_count: 1 
            }; 
            user1.email = String::from("anotheremail@example.com"); 
    }

    {
        // Create user using function without field init shorthand 
        fn build_user(email: String, username: String) -> User {
            User {
                active: true, 
                username: username, 
                email: email, 
                sign_in_count: 1, 
            }
        }
    }

    {
        // Create user using function with field init shorthand 
        fn build_user(email: String, username: String) -> User {
            User {
                active: true, 
                username, 
                email, 
                sign_in_count: 1, 
            }
        }
    }

    {
        // Copy from another instance 
        let user1 = User {
            active: true, 
            username: String::from("someusername123"), 
            email: String::from("someone@example.com"), 
            sign_in_count: 1, 
        };

        // Without Struct Update Syntax...
        // let user2 = User {
        //     active: user1.active, 
        //     username: user1.username, 
        //     email: String::from("another@example.com"), 
        //     sign_in_count: user1.sign_in_count 
        // }; 

        // ...and with Struct Update Syntax 
        let user2 = User {
            email: String::from("another@example.com"), 
            ..user1 
        }; 

        println!("{}", user2.username);
        // Note: the following wouldn't compile 
        // println!("{}", user1.username)
        // as the String username was moved to user2,
        // the new owner of that string. However, the other fields 
        // (bool, and int, as well as the email) are still valid for user1   
    }

    // Tuple structs 
    {
        // Without name fields to create different types 
        struct Color(i32, i32, i32); 
        struct Point(i32, i32, i32); 

        let black = Color(0, 0, 0); 
        let origin = Point(0, 0, 0); 
    }

    {
        // Unit-like structs without any field 
        struct AlwaysEqual;  
        let subject = AlwaysEqual; 
    }

    // Example program using Structs 
    {
        // Using separate variables 
        fn area(width: u32, height: u32) -> u32 {
            width * height 
        }

        let width1 = 30; 
        let height1 = 50; 
        println!(
            "The area of the rectangle is {} square pixels.", 
            area(width1, height1)
        ); 
    }

    {
        // Refactoring with Tuples 
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1 
        }

        let rect1 = (30, 50); 
        println!(
            "The area of the rectangle is {} square pixels.", 
            area(rect1)
        ); 
    }

    
    {
        // Refactoring with Rectangle struct 
        struct Rectangle {
            width: u32,  
            height: u32, 
        }

        let rect1 = Rectangle{
            width: 30, 
            height: 50, 
        }; 

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height 
        }

        println!(
            "The area of the rectangle is {} square pixles.", 
            area(&rect1)
        ); 
    }

    // Print rectangle information
    {   
        #[derive(Debug)]        // -> implementing the Debug trait
        struct Rectangle {
            width: u32,  
            height: u32, 
        }

        let scale = 2; 
        let rect1 = Rectangle{
            width: dbg!(30 * scale), // -> Here dbg takes and return ownership of 30 * scale
            height: 50, 
        }; 

        println!("Rectangle is {:?}", rect1); 
        println!("Rectangle is {:#?}", rect1); 
        dbg!(&rect1);       // -> Here dbg does not take ownership of rect1 as we are passing a reference
    }

    // Method Syntax 
    {   
        // Defining methods for Rectangle 
        #[derive(Debug)]       
        struct Rectangle {
            width: u32,  
            height: u32, 
        }

        // Note: for a struct, we can have multiple impl blocks 
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn width(&self) -> bool {
                self.width > 0
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }

            // Similar to @staticmethod in python 
            fn square(size: u32) -> Self {
                Self {
                    width: size, 
                    height: size, 
                }
            }
        }
        
        let rect1 = Rectangle {
            width: 30, 
            height: 50, 
        }; 

        println!(
            "The area of the rectangle is {} square pixels.", 
            rect1.area()
        ); 

        println!(
            "The rectangle has non-zero width; it is {}", 
            rect1.width()
        ); 

        // Methods with multiple parameters 
        let rect1 = Rectangle {
            width: 30, 
            height: 50, 
        }; 

        let rect2 = Rectangle {
            width: 10, 
            height: 40, 
        };

        let rect3 = Rectangle {
            width: 60, 
            height: 45, 
        };

        println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2)); 
        println!("rect1 can hold rect3: {}", rect2.can_hold(&rect3)); 

        // Using associated functions 
        let square = Rectangle::square(3); 
        dbg!(&square); 
    }
}








