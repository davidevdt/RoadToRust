fn main() {

    // Functions 
    {
        // Functions and custom functions
        println!("Hello, world!");
        another_function();

        fn another_function() {
            println!("Another function.")
        }

    }
    
    {
        // Functions with inputs 
        another_function_2(5); 
        print_labeled_measurement(5, 'h'); 

        fn another_function_2(x: i32) {
            println!("The value of x is: {x}"); 
        }

        fn print_labeled_measurement(value: i32, unit_label: char) {
            println!("The measurement is: {value}{unit_label}"); 
        }
    }

    {
        // Statemenets 
        // let x = (let y = 6); // -> Will return an error 
    }

    {
        // Expressions 
        let y = {
            let x = 3; 
            x + 1               // -> No semicolon at the end, otherwise it would become a statement 
        }; 
        println!("The value of y is: {y}"); 
    }


    {
        // Functions with Return Values 
        let x = five(); 
        println!("The value of x is: {x}"); 

        let x = plus_one(5); 
        println!("The value of x is: {x}"); 

        fn five() -> i32 {
            5
        }
        
        fn plus_one(x: i32) -> i32 {
            x + 1 
        }
    }
}







