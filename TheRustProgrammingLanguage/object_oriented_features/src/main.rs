fn main() {
    
    // Encapsulation 
    {
        pub struct AveragedCollection {
            list: Vec<i32>, 
            average: f64, 
        }

        impl AveragedCollection {
            pub fn add(&mut self, value: i32) {
                self.list.push(value); 
                self.update_average(); 
            }

            pub fn remove(&mut self) -> Option<i32> {
                let result = self.list.pop(); 
                match result {
                    Some(value) => {
                        self.update_average();
                        Some(value)
                    }
                    None => None,
                }
            }

            pub fn average(&self) -> f64 {
                self.average
            }

            fn update_average(&mut self) {
                let total: i32 = self.list.iter().sum(); 
                self.average = total as f64 / self.list.len() as f64; 
            } 

        }
    }

    // Using trait objects that allow for values of different types 
    {
        pub trait Draw {
            fn draw(&self); 
        }

        pub struct Screen {
            pub components: Vec<Box<dyn Draw>>, 
        }

        impl Screen {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw(); 
                }
            }
        }
        
        // Alternative implementation with generics 
        // - this restricts the components vector 
        // - to work only wit one type, e.g. only Button's or only Textfield's
        // - and is more useful when we need to work with a homogeneous type 
        // pub struct Screen<T: Draw> {
        //     pub components: Vec<T>, 
        // }

        // impl<T> Screen<T> 
        // where 
        //     T: Draw, 
        // {
        //     pub fn run(&self) {
        //         for component in self.components.iter() {
        //             component.draw(); 
        //         }
        //     }
        // }

        // Let's now implement the trait: 
        pub struct Button {
            pub width: u32, 
            pub height: u32, 
            pub label: String, 
        }

        impl Draw for Button {
            fn draw(&self) {
                // code to actually draw a button 
            }
        }

        // use gui::Draw; 
        struct SelectBox {
            width: u32, 
            height: u32, 
            options: Vec<String>, 
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                // code to actually draw a select bo 
            }
        }

        // use gui::{Button, Screen}; 
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75, 
                    height: 10, 
                    options: vec![
                        String::from("Yes"), 
                        String::from("Maybe"), 
                        String::from("No"), 
                    ],
                }), 
                Box::new(Button {
                    width: 50, 
                    height: 10, 
                    label: String::from("OK"),
                }),
            ],
        };
        screen.run(); 
    }

    // Implementing an Object-Oriented design pattern - The state pattern 

    // ---------------------------------------------------------------------
    // We want to achieve being able to run this: 

    // use blog::Post;
    // let mut post = Post::new();

    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());
    // ---------------------------------------------------------------------

    pub struct Post {
        state: Option<Box<dyn State>>, 
        content: String, 
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})), 
                content: String::new(), 
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self) 
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>; 
        fn approve(self: Box<Self>) -> Box<dyn State>; 
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self 
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self 
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    // Alternative Rust implementation 
    {

        // This will lead to the following client calls: 
        // use blog::Post;
        // let mut post = Post::new();
        // post.add_text("I ate a salad for lunch today");
        // let post = post.request_review();
        // let post = post.approve();
        // assert_eq!("I ate a salad for lunch today", post.content());

        pub struct Post {
            content: String, 
        }

        pub struct DraftPost {
            content: String, 
        }

        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new(), 
                }
            }

            pub fn content(&self) -> &str {
                &self.content
            }
        }

        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text); 
            }

            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content, 
                }
            }
        }

        pub struct PendingReviewPost {
            content: String, 
        }

        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }
        
    }

}