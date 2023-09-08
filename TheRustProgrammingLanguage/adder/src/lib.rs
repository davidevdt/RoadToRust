pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Make a test fail on purpose 
    // #[test]
    // fn another() {
    //     panic!("Make this test fail"); 
    // }

    // With custom struct 
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8, 
            height: 7, 
        }; 
        let smaller = Rectangle {
            width: 5, 
            height: 1, 
        }; 

        assert!(larger.can_hold(&smaller)); 
    }

    // Check inequality 
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8, 
            height: 7, 
        }; 
        let smaller = Rectangle {
            width: 5, 
            height: 1, 
        }; 

        assert!(!smaller.can_hold(&larger)); 
    }

    // With assert_eq and assert_ne
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2)); 
    }

    #[test]
    fn it_doesnt_add_two() {
        assert_ne!(5, add_two(2)); 
    }

    // With custom failure message
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol"); 
    //     assert!(
    //         result.contains("Carol"), 
    //         "Greeting did not contain name, value was {}", 
    //         result
    //     ); 
    // }

    // Checking with should_panic
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200); 
    }

    // With Result<T, E> in tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // To check if it's an error: assert!(value.is_err())

    // Ignore a test 
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2)); 
    }
}
