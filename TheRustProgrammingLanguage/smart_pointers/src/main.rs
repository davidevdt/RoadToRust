fn main() {
    
    // Box<T> to Point data on the Heap 
    {
        // Store data on the heap 
        let b = Box::new(5); 
        println!("b = {}", b); 

        // A cons list definition : it won't compile as we don't specify a list size, so it's taken as "infinite-size" list 
        // enum List {
        //     Cons(i32, List), 
        //     Nil, 
        // }
        // 
        // use crate::List::{Cons, Nil}; 
        // let list = Cons(1, Cons(2, Cons(3, Nil))); 

        // This will compile - as Rust knows the pointer's size 
        enum List {
            Cons(i32, Box<List>), 
            Nil, 
        }

        use List::{Cons, Nil}; 
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); 
        
    }

    // Smart Pointers and the Deref trait 
    {
        // References are type of pointers 
        let x = 5; 
        let y = &x; 
        assert_eq!(5, x); 
        assert_eq!(5, x); 

        // Using Box<T> as a reference 
        let x = 5; 
        let y = Box::new(x);    // -> points to a copied value of x, rather than being a reference to it 
        assert_eq!(5, x); 
        assert_eq!(5, *y); 

        // Custom smart pointer 
        // Note: unlike Box, MyBox stores values on the stack 
        // just for illustrative purposes 
        struct MyBox<T>(T);             // -> tuple struct 

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        use std::ops::Deref; 
        impl<T> Deref for MyBox<T> {
            type Target = T; 

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let x = 5; 
        let y = MyBox::new(x); 
        assert_eq!(5, x); 
        assert_eq!(5, *y);      // -> behind the scenes Rust runs: *(y.deref()) 

        // Implicit deref coercions 
        fn hello(name: &str) {
            println!("Hello, {name}!")
        }

        let m = MyBox::new(String::from("Rust")); 
        hello(&m);      // --> Deref coercion 
        // Without implementing deref coercion (impl deref above): 
        // hello(&(*m)[..]); 

        // How deref coercion interacts with mutability: 
        // From &T to &U when T: Deref<Target=U>
        // From &mut T to &mut U when T: DerefMut<Target=U>
        // From &mut T to &U when T: Deref<Target=U>
        // but: immutable references will never coerce to mutable ones 
    }

    // The Drop trait 
    {
        struct CustomSmartPointer {
            data: String, 
        }

        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data); 
            }
        }

        {
            let c = CustomSmartPointer {
                data: String::from("my stuff"), 
            }; 

            let d = CustomSmartPointer {
                data:String::from("other stuff"), 
            }; 

            println!("CustomSmartPointer created."); 
        }

        // Dropping a value early with std::mem::drop 
        let c = CustomSmartPointer {
            data: String::from("some data"),  
        }; 
        println!("CustomSmartPointer created."); 
        // c.drop();        // -> error: can't manually call drop as Rust will do it already when it goes out of scope 
        drop(c);            // -> std::mem::drop  
        println!("CustomSmartPointer dropped before the end of main."); 
    }

    // Rc<T> , the reference count smart pointer 
    {
        // Used to share data 
        // This won't compile:
        // enum List {
        //     Cons(i32, Box<List>),
        //     Nil,
        // }
        
        // use List::{Cons, Nil}; 
        // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        // let b = Cons(3, Box::new(a)); // -> a moved into b; b owns a 
        // let c = Cons(4, Box::new(a)); // -> not allowed: a is an invalid reference 

        // This will compile: 
        enum List {
            Cons(i32, Rc<List>), 
            Nil, 
        }

        use List::{Cons, Nil}; 
        use std::rc::Rc; 
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); 
        let b = Cons(3, Rc::clone(&a));     // clone doesn't create a deep copy: just increases reference count 
        let c = Cons(4, Rc::clone(&a)); 

        // Increasing the reference count cloning an Rc<T> 
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); 
        println!("count after creating a = {}", Rc::strong_count(&a)); 
        let b = Cons(3, Rc::clone(&a)); 
        println!("count after creating b = {}", Rc::strong_count(&a)); 
        {
            let c = Cons(4, Rc::clone(&a)); 
            println!("count after creating c = {}", Rc::strong_count(&a)); 
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a)); 
    }

    // RefCell<T> and the Interior Mutability Pattern 
    {
        // Interior Mutability : A mutable borrow to an immutable value 
        // -> Because of the borrowing rules, this code won't compile: 
        // let x = 5; 
        // let y = &mut x; 

        // Example: Mock Objects 
        pub trait Messenger {
            fn send(&self, msg: &str);  // -> note that send()'s first argument is &self, not &mut self
        } 

        pub struct LimitTracker<'a, T: Messenger> {
            messenger: &'a T, 
            value: usize, 
            max: usize, 
        }

        impl <'a, T> LimitTracker<'a, T> 
        where 
            T: Messenger
        {
            pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
                LimitTracker {
                    messenger, 
                    value: 0, 
                    max, 
                }
            }

            pub fn set_value(&mut self, value: usize) {
                self.value = value; 
                let percentage_of_max = self.value as f64 / self.max as f64; 

                if percentage_of_max >= 1.0 {
                    self.messenger.send("Error: You are over your quota!");
                } else if percentage_of_max >= 0.9 {
                    self.messenger 
                        .send("Urgent warning: You've used up over  90% of your quota!"); 
                } else if percentage_of_max >= 0.75 {
                    self.messenger
                        .send("Warning: You've used up over 75% of your quota!"); 
                }
            }
        }

        // Now let's say that we want to test it, but we need to create a mock object 
        // for the messenger: 
        // #[cfg(test)]
        // mod tests {
        //    use super::*; 

            struct MockMessenger {
                // sent_messages: Vec<String>, 
                sent_messages: RefCell<Vec<String>>, 
            }

            impl MockMessenger {
                fn new() -> MockMessenger {
                    MockMessenger {
                        // sent_messages: vec![],
                        sent_messages: RefCell::new(vec![]),
                    }
                }
            }

            impl Messenger for MockMessenger {
                fn send(&self, message: &str) {
                    // self.sent_messages.push(String::from(message)); 
                    self.sent_messages.borrow_mut().push(String::from(message)); 
                }
            }

            #[test]
            fn it_sends_an_over_75_percent_warning_message() {
                let mock_messenger = MockMessenger::new(); 
                let mut limit_tracker = LimitTracker::new(&mock_messenger, 100); 
                limit_tracker.set_value(80); 

                // assert_eq!(mock_messenger.sent_messages.len(), 1); 
                assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); 
            }
        // }

        // borrow_mut : gets mutable reference; borrow: gets immutable reference 

        // Keeping track of borrows at runtime 
        // -> Invalid code: not possible to have two mutable references 
        // impl Messenger for MockMessenger {
        //     fn send(&self, message: &str) {
        //         let mut one_borrow = self.sent_messages.borrow_mut();
        //         let mut two_borrow = self.sent_messages.borrow_mut();
    
        //         one_borrow.push(String::from(message));
        //         two_borrow.push(String::from(message));
        //     }
        // }

        // Multiple owners of mutable data 
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};
        use std::cell::RefCell;
        use std::rc::Rc;

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);

    }

    // Reference cycles can leak memory 
    {
        // Creating a reference cycle 
        use List::{Cons, Nil}; 
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>), 
            Nil, 
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item), 
                    Nil => None, 
                }
            }
        }

        // Now a reference cycle is created: 
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))); 
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());

        // Preventing reference cycle: turning an Rc<T> into a Weak<T> 
        // Rc::downgrade on an Rc<T> produces a Weak<T> pointer, which has a weak_count 
        // weak_count doesn't need to be 0 for the Rc<T> to be cleaned up 
        // Calling an `upgrade` method on a Weak<T> will return an Option<Rc<T>>

        // A tree data structure: a Node with Child nodes 
        // use std::cell::RefCell; 
        // use std::rc::{Rc, Weak}; 
        use std::rc::Weak; 

        #[derive(Debug)]
        struct Node {
            value: i32, 
            children: RefCell<Vec<Rc<Node>>>, 
            parent: RefCell<Weak<Node>>, // -> it cannot be a Rc<T> otherwise we would create a reference cycle
                                            // -> a parent node should own its children: if it's dropped children should be 
                                            // -> dropped too, but opposite not true: dropping a child does not drop the parent
        }

        let leaf = Rc::new(Node {
            value: 3, 
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]), 
        }); 

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 

        let branch = Rc::new(Node {
            value: 5, 
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()), 
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); 

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 

        // Visualizing changes to strong_count and weak_count 
        let leaf = Rc::new(Node {
            value: 3, 
            parent: RefCell::new(Weak::new()), 
            children: RefCell::new(vec![]), 
        }); 

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), 
            Rc::weak_count(&leaf), 
        ); 

        {
            let branch = Rc::new(Node {
                value: 5, 
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]), 
            }); 

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch); 

            println!(
                "branch strong = {}, weak = {}", 
                Rc::strong_count(&branch), 
                Rc::weak_count(&branch), 
            ); 

            println!(
                "leaf strong = {}, weak = {}", 
                Rc::strong_count(&leaf), 
                Rc::weak_count(&leaf), 
            )
        }

        println!(
            "leaf parent = {:?}", leaf.parent.borrow().upgrade()
        ); 
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), 
            Rc::weak_count(&leaf) 
        )
    }

}
