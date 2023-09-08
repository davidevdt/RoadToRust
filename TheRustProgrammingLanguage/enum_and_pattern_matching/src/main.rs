fn main() {
    
    // Enums
    {
        // Define enums 
        enum IpAddrKind {
            v4, 
            v6
        }

        // Enum values 
        let four = IpAddrKind::v4; 
        let six = IpAddrKind::v6; 

        // Call function with enum as argument 
        // route(IpAddrKind::v4);
        // route(UpAddrKind::v6); 
        // ... for a possible rout function:  
        //fn route(ip_kind: IpAddrKind) { }

    }

    // Store enum data
    {
        // a. With structs
        enum IpAddrKind {
            v4, 
            v6
        }

        struct IpAddr {
            kind: IpAddrKind, 
            address: String, 
        }         

        let home = IpAddr {
            kind: IpAddrKind::v4, 
            address: String::from("127.0.0.1"),
        }; 
        
        let loopback = IpAddr {
            kind: IpAddrKind::v6, 
            address: String::from("::1"),    
        }; 

        // b1. Putting data directly in the enum variant 
        enum IpAddr2 {
            v4(String),
            v6(String)
        }

        let home = IpAddr2::v4(String::from("127.0.0.1"));  
        let loopback = IpAddr2::v6(String::from("::1")); 

        // b2. Using multiple data 
        enum IpAddr3 {
            v4(u8, u8, u8, u8), 
            v6(String), 
        }

        let home = IpAddr3::v4(127, 0, 0, 1); 
        let loopback = IpAddr3::v6(String::from("::1")); 

        // Note: you can also put custom struct types in the enum items
        // e.g.: 
        // enum IpAddr {
        //  v4(MyStructForV4),
        //  v6(MyStructrForV2), 
        // }
    } 

    {
        // Methods for Enum's

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        // The latter is equivalent to: 
        // struct QuitMessage; 
        // Struct MoveMessage {
        //     x: i32, 
        //     y: i32, 
        // }
        // struct WriteMessage(String); // tuple struct 
        // struct ChangeColor(i32, i32, i32); // tuple struct 

        // Methods for enums 
        impl Message {
            fn call(&self) {
                // method body 
            }
        }
        let m = Message::Write(String::from("Hello")); 
        m.call(); 
    }

    // Option Enum 
    {
        // Implemented as:
        // enum Option<T> {
        //     None, 
        //     Some(T),     
        // }

        // let some_number = Some(5); // type: Option<i32>
        // let some_char = Some('e'); // type: Option<char> 
        // let absent_number:  Option<i32> = None; // -> Option<type> is required here 

        // This won't compile: 
        // let x: i8 = 5; 
        // let y: Option<i8> = Some(5); 
        // let sum = x + y;        

        // This will compile:         
        let x: i8 = 5; 
        let y = Some(5); 
        let sum = x + y.unwrap(); 
        println!("Sum is {}", sum); 
        println!("Y = {}", y.unwrap()); 
    }
     
    // The match control flow 
    {

        enum Coin {
            Penny, 
            Nickel, 
            Dime, 
            Quarter, 
        }

        fn value_in_cents(coin: Coin) -> u8 {
            // Note: the code associated with each arm is an expression 
            // Note 2: In match, we can't omit any value of the enum it refers to
            match coin {
                Coin::Penny => 1, 
                Coin::Nickel => 5, 
                Coin::Dime => 10, 
                Coin::Quarter => 25,
            }
        }

        // With multiple lines of code for an arm: 
        fn value_in_cents_2(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Luck penny!"); 
                    1   // note that 1 is an expression here 
                } // -> the comma here is optional 
                Coin::Nickel => 5, 
                Coin::Dime => 10, 
                Coin => 25,
            }
        }

        // With patterns binded to a value 
        #[derive(Debug)]
        enum UsState {
            Alambama, 
            Alaska, 
            // Others 
        }

        enum Coin2 {
            Penny, 
            Nickel, 
            Dime, 
            Quarter(UsState), 
        }

        fn value_in_cents_3(coin: Coin2) -> u8 {
            match coin {
                Coin2::Penny => 1, 
                Coin2::Nickel => 5, 
                Coin2::Dime => 10, 
                Coin2::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }

        value_in_cents_3(Coin2::Quarter(UsState::Alaska)); 
    }

    // Matching with Option<T>
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None, 
                Some(i) => Some(i + 1), 
            }
        }

        let five = Some(5); 
        let six = plus_one(five); 
        let none = plus_one(None); 
    }

    // Match with default value 
    {
        let dice_roll = 9; 
        match dice_roll {
            3 => add_fancy_hat(), 
            7 => remove_fancy_hat(), 
            other => move_player(other), // other is the name for the default variant arm 
                                         // This last pattern will match all other unlisted possiblities 
                                         // "Catch-all pattern", it always have to be last (otherwise other arms won't run)
        }

        fn add_fancy_hat(){}
        fn remove_fancy_hat(){}
        fn move_player(num_spaces: u8) {}
    }

    // Catch-all with _ placeholder 
    {
        let dice_roll = 9; 
        match dice_roll {
            3 => add_fancy_hat(), 
            7 => remove_fancy_hat(), 
            _ => reroll() // Exhaustivity requirement is fullfilled; all other values in the arm are ignored via _ 
        }

        fn add_fancy_hat(){}
        fn remove_fancy_hat(){}
        fn reroll() {}
    }

    // Catch-all with _ placeholder and no action 
    {
        let dice_roll = 9; 
        match dice_roll {
            3 => add_fancy_hat(), 
            7 => remove_fancy_hat(), 
            _ => () // Using the unit value () 
        }

        fn add_fancy_hat(){}
        fn remove_fancy_hat(){}
    }

    // Control flow with if-let 
    {
        // Instead of using unit value...
        let config_max = Some(3u8); 
        match config_max {
            Some(max) => println!("The maximum is configured to be {}", max), 
            _ => ()
        }

         // ...use if-let:  
        let config_max = Some(3u8); 
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max); 
        }
    }

    {   
        // if-let with else 
        // Example: count all non-quarter coins
        #[derive(Debug)]
        enum UsState {
            Alambama, 
            Alaska, 
            // Others 
        }

        enum Coin {
            Penny, 
            Nickel, 
            Dime, 
            Quarter(UsState), 
        }

        // We could either do this...
        // let mut count = 0; 
        // let coin = Coin::Quarter(UsState::Alambama); 
        // match coin {
        //     Coin::Quarter(state) => println!("State quarter from {:?}!", state), 
        //     _ => count += 1,
        // }

        // ... or we could do this: 
        let mut count = 0; 
        let coin = Coin::Quarter(UsState::Alambama); 
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state); 
        } else {
            count += 1; 
        }
    }
}


