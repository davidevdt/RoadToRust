use std::io; 

fn main() {
    
    // Immutability and mutability 
    {
        // Immutable variables by default 
        let x = 5; 
        println!("Immutable -- The value of x is: {x}"); 
        // x = 6;  // -> This will cause a compile error 

        // Mutable variables 
        let mut x = 5; 
        println!("Mutable -- The value of x (before) is: {x}"); 
        x = 6;  // -> This will compile 
        println!("Mutable -- The value of x (after) is: {x}"); 
    } 

    // Constants 
    {
        // Differences between constants and immutable variables: 
        // * can't use mut with constants 
        // * constants can be declared in any scope, including global scope 
        // * constants may be set only to constant expression, not at compile time 
        const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3; 
        println!("Constant -- The numbe of seconds in three hours is: {THREE_HOURS_IN_SECONDS}"); 
    }

    // Shadowing 
    {
        let x = 5; 
        let x = x + 1; 
        {
            let x = x * 2; 
            println!("Shadowing -- The value of x in the inner scope is: {x}"); // -> Will print 12
        }
        println!("Shadowing -- The value of x is: {x}"); // -> Will print 6 
    }


    // Shadowing -- Mutable vs. immutable 
    {
        // * If we reassign a value to a mut variable with let, we'll get compile error
        // * To immutable variables, we can reassign with let and change the type of value 

        // Immutable: 
        let spaces = "    "; 
        let spaces = spaces.len(); // -> Can change type 
        println!("Shadowing (Immutablle variable) -- The number of spaces is: {spaces}"); 

        // Mutable: 
        // let mut spaces = "    ";  
        // let spaces = spaces.len(); // -> Compile error [can't mutate variable's type]


    }   

    // Data Types 

    {
        // Scalar Types: integers, floating-point, Booleans, characters 

        // Integer Types [-(2^(n-1)) to (2^(n-1))-1 signed; 0 to (2^n)-1 unsigned; n = number of bits]
        // LENGTH   | SIGNED    | UNSIGNED  |
        // 8-bit    | i8        | u8        |
        // 16-bit   | i16       | u16       |
        // 32-bit   | i32       | u32       |   -> default: i32
        // 64-bit   | i64       | u64       |
        // 128-bit  | i128      | u128      |
        // arch     | isize     | usize     |

        // Integer Literals
        // Number literals  | Example       |
        // Decimal          | 98_222        |
        // Hex              | 0xff          |
        // Octal            | 0o77          |
        // Binary           | 0b1111_0000   |
        // Byte (u8 only)   | b'A'          | 

        // Floating types (two types: f64 (default) and f32)    -> default: f64 
        // let x = 2.0; // -> f64 
        // let y: f32 = 3.0; // -> f32 

        // Numeric Operations 
        // let sum = 5 + 10; 
        // let difference = 95.5 - 4.3;  
        // let product = 4 * 30; 
        // let quotient = 56.7 / 32.2; 
        // let truncated = -5 / 3; // -> = -1
        // let remainder = 43 % 5; 

        // Boolean [1 byte]
        // let t = true; 
        // let f: bool = false; // with explicit type annotation

        // Character 
        // let c = 'z';
        // let z: char = 'Z'; // with explicit type annotation
        // let smiling_cat = '🐱';
    }

    {
        // Compound Types: tuples, arrays 
    
        // Tuple Type (fixed size, variety of types)
        // let tup: (i32, f64, u8) = (500, 6.4, 1);

        // Access tuple value by pattern matching 
        let tup = (500, 6.4, 1); 
        let (_x, y, _z) = tup; 
        println!("The value of y is: {y}");  
    
        // Access tuple value by indexing 
        // let x: (i32, f64, u8) = (500, 6.4, 1); 
        // let five_hundred = x.0; 
        // let six_point_four = x.1; 
        // let one = x.2; 

        // Unit tuple (empty value - empty return type) -> expressed as ()

        // Array Type (fixed length, same type); data allocated on the stack 
        // let a = [1, 2, 3, 4, 5];
        // let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]; 
        
        // With type annotation:
        // let a: [i32; 5] = [1, 2, 3, 4, 5];  
        
        // Array initialization: 
        // let a = [3; 5]; // ->same as: let a = [3, 3, 3, 3, 3]; 

        // Access array value by indexing 
        // let a = [1, 2, 3, 4, 5]; 
        // let first = a[0]; 
        // let second = a[1]; 

        // Invalid array element access 
        // The following will throw a runtime error if the index is out of bounds 
        let a = [1, 2, 3, 4, 5]; 
        println!("Please enter an array index...");

        let mut index = String::new(); 
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line"); 
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered not a number!"); 
        let element = a[index]; 
        println!(
            "The value of the element at index {index} is: {element}"
        ); 
    }
}
