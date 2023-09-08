fn main() {
    
    // Closures 
    {
        // Capturing the environment with closures 
        #[derive(Debug, PartialEq, Copy, Clone)]
        enum ShirtColor {
            Red, 
            Blue, 
        }

        struct Inventory {
            shirts: Vec<ShirtColor>, 
        }

        impl Inventory {
            fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
                user_preference.unwrap_or_else(|| self.most_stocked())
            }

            fn most_stocked(&self) -> ShirtColor {
                let mut num_red = 0; 
                let mut num_blue = 0; 

                for color in &self.shirts {
                    match color {
                        ShirtColor::Red => num_red += 1, 
                        ShirtColor::Blue => num_blue += 1, 
                    }
                }

                if num_red > num_blue {
                    ShirtColor::Red
                } else {
                    ShirtColor::Blue 
                }
            }
        }

        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue], 
        }; 

        let user_pref1 = Some(ShirtColor::Red); 
        let giveaway1 = store.giveaway(user_pref1); 
        println!(
            "The user with preference {:?} gets {:?}", 
            user_pref1, giveaway1
        );

        let user_pref2 = None; 
        let giveaway2 = store.giveaway(user_pref2); 
        println!(
            "The user with preference {:?} gets {:?}", 
            user_pref2, giveaway2
        );  
    }

    {
        // Storing closure in variable and annotating types 
        // let expensive_closure = |num:u32| -> u32 {
        //     println!("calculating slowly...");
        //     thread::sleep(Duration::from_secs(2)); 
        //     num
        // }

        // Closures' syntax is similar to functions' syntax 
        fn add_one_v1 (x: u32) -> u32 {x + 1}
        let add_one_v2 = |x: u32| -> u32 {x + 1}; 
        // let add_one_v3 = |x| {x + 1}; 
        // let add_one_v4 = |x| x + 1; 

        // Without type annotation and calling with different types
        let example_closure = |x| x; 
        let s = example_closure(String::from("Hello")); 
        // let n = example_closure(5); // -> this won't compile: the inferred type for this closure's input was String  
    }

    {
        // Capturing references or moving ownerships 

        // Capturing immutable reference: 
        let list = vec![1, 2, 3]; 
        println!("Before defining closure: {:?}", list); 

        let only_borrows = || println!("From closure: {:?}", list); 

        println!("Before calling closure: {:?}", list); 
        only_borrows(); 
        println!("After calling closure: {:?}", list); 

        // Capturing a mutable reference: 
        let mut list = vec![1, 2, 3]; 
        println!("Before defining closure: {:?}", list); 

        let mut borrows_mutably = || list.push(7); 
        // println!("Before calling mutable closure: {:?}", list); // -> immutable print not allowed
        borrows_mutably(); 
        println!("After calling closure: {:?}", list); 

        // Taking ownership 
        use std::thread; 
        let list = vec![1, 2, 3]; 
        println!("Before defining closure: {:?}", list); 

        thread::spawn(move || println!("From thread: {:?}", list))  // -> move is necessary as main thread might end before this thread
            .join()
            .unwrap()
    }

    {
        // FnOnce, FnMut, and Fn traits 

        // Definition of unwrap_or_else method: 
        // impl<T> Option<T> {
        //     pub fn unwrap_or_else<F>(self, f: F) -> T 
        //     where 
        //     F: FnOnce() -> T    // f must be able to be called once, (takes no arguments), and returns a T
        //     {
        //         match self {
        //             Some(x) => x, 
        //             None => f(), 
        //         }
        //     }    
        // }

        // Note: we can use any function we want in unwrap or else; from the book: 
        // on an Option<Vec<T>> value, we could call unwrap_or_else(Vec::new) 
        // to get a new, empty vector if the value is None

        // With sort_by_key: 
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let mut list = [
            Rectangle {width: 10, height: 1 }, 
            Rectangle {width: 3, height: 5 }, 
            Rectangle { width: 7, height: 12},
        ]; 

        list.sort_by_key(|r| r.width); // -> closures implements FnMut 
        println!("{:#?}", list); 

        // This won't compile, the closure implements FnOnce: 
        // let mut sort_operations = vec![]; 
        // let value = String::from("by key called"); 
        // list.sort_by_key(|r| {  // -> this closure can be called once, but the second time won't work as "value" won't be in the environment to be pushed into sort_operations again
        //     // sort_operations.push(value); 
        //     // sort_operations.push(&value); // -> but with this it will work! 
        //     r.width
        // }); 
        // println!("{:#?} with: {:?}", list, sort_operations); 

        // To fix it, we need to fix it so that it doesn't move values out of the environment 
        let mut num_sort_operations  = 0; 
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        }); 
        println!("{:?}, sorted in {num_sort_operations} operations", list); 

    }

    // Iterators 
    {
        // Iterators are lazy 
        {
            let v1 = vec![1, 2, 3]; 
            let v1_iter = v1.iter(); // -> this code by itself doesn't do anything useful until we consume the iterator 
        }

        let v1 = vec![1, 2, 3]; 
        let v1_iter = v1.iter(); 
        for val in v1_iter {
            println!("Got: {}", val); 
        }
    }

    {
        // Iterator trait and next method 
        // pub trait Iterator {
        //     type Item; 
        //     fn next(&mut self) -> Option<Self::Item>; 
        //     // methods with default implementations elided 
        // }

        #[test]
        fn iterator_demonstation() {
            let v1 = vec![1, 2, 3]; 
            let mut v1_iter = v1.iter(); // it must be mut as we are calling next, which consumes the internal state of the iterator
                                         // not needed in for loops as they take ownership and make it mutable behind the scenes 
            assert_eq!(v1_iter.next(), Some(&1)); 
            assert_eq!(v1_iter.next(), Some(&2)); 
            assert_eq!(v1_iter.next(), Some(&3)); 
            assert_eq!(v1_iter.next(), None); 

            // into_iter() : produces owned values, rather than immutable references 
            // iter_mut() : produces mutable references 
        }
    }

    {
        // Methods that consume iterators (consuming adaptors)
        #[test]
        fn iterator_sum() {
            let v1 = vec![1, 2, 3]; 
            let v1_iter = v1.iter(); 
            let total: i32 = v1_iter.sum(); 
            assert_eq!(total, 6); 
            // here it is not possible to call v1_iter anymore as it was consumed by sum 
        }
    }

    {
        // Methods that produce other iterators (iterator adaptors)
        // They produce new iterators by changing some aspect of the original one
        let v1: Vec<i32> = vec![1, 2, 3]; 
        // v1.iter().map(|x| x + 1);
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); 
        assert_eq!(v2, vec![2, 3, 4]);  
    }

    {
        // Using closures that capture their environment
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }

        fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }

        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe {
                    size: 10, 
                    style: String::from("sneaker"), 
                },
                Shoe {
                    size: 13, 
                    style: String::from("sandal"), 
                }, 
                Shoe {
                    size: 10, 
                    style: String::from("boot"), 
                }
            ];

            let in_my_size = shoes_in_size(shoes, 10); 

            assert_eq!(
                in_my_size, 
                vec![
                    Shoe {
                        size: 10, 
                        style: String::from("sneaker")
                    }, 
                    Shoe {
                        size: 10, 
                        style: String::from("boot")
                    },
                ]
            ); 
        }
    }
}
