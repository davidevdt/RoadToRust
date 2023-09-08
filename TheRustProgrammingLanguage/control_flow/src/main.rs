use std::io; 

fn main() {

    // If expressions
    {
        let number = 7; 
        if number < 5 {
            println!("Condition was true"); 
        } else {
            println!("Conidtion was false")
        }

        let number = 6; 
        if number % 4 == 0 {
            println!("number is divisible by 4"); 
        } else if number % 3 == 0 {
            println!("number is divisible by 3"); 
        } else if number % 2 == 0 {
            println!("number is divisible by 2"); 
        } else {
            println!("number is not divisible by 4, 3, or 2"); 
        }
    }

    {
        // Declare a variable with an if statement 
        let condition = true; 
        let number = if condition {5} else {6}; 
        println!("The value of number is: {number}"); 

        // This won't compile as types in the if statement do not match: 
        // let condition = true; 
        // let number = if condition{5} else {"six"}; 
        // println!("The value of number is: {number}"); 
    }

    // Loops 
    {
        // Loop
        let mut counter = 0;
        let result = loop {
            counter += 1; 
            if counter == 10 {
                break counter * 2;
            }
        }; 
        println!("The result is {result}"); 

        // Loop labels
        let mut count = 0; 
        'counting_up: loop {
            println!("Count = {count}"); 
            let mut remaining = 10; 
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break; 
                } 
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1; 
            }
            count += 1; 
        }
        println!("End count = {count}"); 
    }

    // While loops 
    {
        let mut number = 3; 
        while number != 0 {
            println!{"{number}"}; 
            number -= 1; 
        }
        println!("LIFTOFF!!!"); 
    
        // Loop with colletion 
        let a = [10, 20, 30, 40, 50]; 
        let mut index = 0; 
        while index < 5 {
            println!("The value is: {}", a[index]); 
            index += 1; 
        }
    }

    // For loop 
    {
        // For loop with collection 
        let a = [10, 20, 30, 40, 50]; 
        for element in a {
            println!("The value is: {element}"); 
        }

        // For loop with (reversed) range
        for number in (1..4).rev() {
        // for number in (4..1).rev() {
            println!("{number}"); 
        }
        println!("LIFTOFF!!!"); 
    }

    // Exercise: find n-th Fibonacci number
    {
        // Fibonacci number: f(0) = 0; f(1) = 1; f(n) = f(n-2) + f(n-1)
        let mut n = String::new(); 
        let mut result: u64 = 0; 
        let mut fib_2: u64; 
        let mut fib_1: u64 = 0; 

        println!("Please input n...");
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read number."); 
        
        let n: u64 = n
            .trim()
            .parse()
            .expect("Failed to convert to number."); 

        for fib in 0..=n {
            if fib == 0 {
                continue; 
            }

            if fib == 1 { 
                result = fib; 
                continue; 
            }
            
            fib_2 = fib_1; 
            fib_1 = result; 
            result = fib_1 + fib_2; 
        }
        println!("Fib({n}) = {result}"); 
    }
}
