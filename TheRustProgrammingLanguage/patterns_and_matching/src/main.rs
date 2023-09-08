fn main() {
    
    // All the places patterns can be used 
    {
        // match arms 
        // match value {
        //     PATTERN => EXPRESSION, 
        //     PATTERN => EXPRESSION, 
        //     PATTERN => EXPRESSION, 
        // }

        // the pattern _ matches anything but does not bind to a variable (usually last arm)

        // conditional 'if let', 'else if', and 'else if let' expressions
        {
            let favorite_color: Option<&str> = None; 
            let is_tuesday = false; 
            let age: Result<u8, _> = "34".parse(); 

            if let Some(color) = favorite_color {
                println!("Using your favorite color, {color}, as the background"); 
            } else if is_tuesday {
                println!("Tuesday is green day!"); 
            } else if let Ok(age) = age {
                if age > 30 {
                    println!("Using purple as the background color")
                } else {
                    println!("Using orange as the background color"); 
                }
            } else {
                println!("Using blue as the background color") ;
            }
        }

        // while let conditional loops 
        let mut stack = Vec::new(); 
        stack.push(1); 
        stack.push(2); 
        stack.push(3); 

        while let Some(top) = stack.pop() {     // if vector is empty, .pop() returns None and 
            println!("{}", top); 
        }

        // for loops 
        let v = vec!['a', 'b', 'c']; 

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index); 
        }

        // let statements: let PATTERN = EXPRESSION  
        // let x = 5; 
        // let (x, y, z) = (1, 2, 3); 

        // Function parameters 
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y); 
        }

        let point = (3, 5); 
        print_coordinates(&point); 
    }

    // Refutability: whether a pattern might fail to match 
    {
        let some_option_value: Option<i32> = None; 
        // let Some(x) = some_option_value; // -> does not compile! 
                                            // -> We tried to use a refutable pattern when a 
                                            // -> non-refutable one is required 

        // The following is valid: 
        if let Some(x) = some_option_value {
            println!("{}", x); 
        } 

        // This code will generate a warning: we use if let with a pattern that always matches 
        // if let x = 5 {
        //     println!("{}", x); 
        // }

    }

    // Pattern syntax 
    {
        // Matching literals 
        let x = 1; 
        match x {
            1 => println!("one"), 
            2 => println!("two"), 
            3 => println!("three"), 
            _ => println!("anything"), 
        }

        // Matching names variables 
        // Note: "match" will create a new scope and variables declared inside match 
        // will shadow variables with the same name declared outside the match construct 
        let x = Some(5); 
        let y = 10; 

        match x {
            Some(50) => println!("Got 50"),                 // 50 , Some value 
            Some(y) => println!("Matched, y = {y}"),        // any other Some value
            _ => println!("Default case, x = {:?}", x),     // none case
        }

        println!("at the end: x = {:?}, y = {y}", x); 

        // Multiple patterns 
        let x = 1; 
        match x {
            1 | 2 => println!("one or two"), 
            3 => println!("three"), 
            _ => println!("Anything"), 
        }

        // Matching ranges with ..= 
        let x = 5; 
        match x {
            1..=5 => println!("one through five"), 
            _ => println!("something else"), 
        }

        let x = 'c'; 
        match x {
            'a'..='j' => println!("early ASCII letter"), 
            'k'..='z' => println!("late ASCII letter"), 
            _ => println!("Something else"), 
        }

        // Destructuring to break apart values 
        // (a)
        struct Point {
            x: i32, 
            y: i32, 
        }

        let p = Point{x: 0, y: 7}; 
        let Point {x: a, y: b} = p; // valid, but lot of duplicated code 
        // let Point{ x, y } = p; // -> shortcut, also valid 
        assert_eq!(0, a); 
        assert_eq!(7, b); 

        // (b)
        let p = Point{ x: 0, y: 7 }; 
        match p {
            Point {x, y: 0} => println!("On the x axis is at {x}"), 
            Point {x: 0, y} => println!("On the y axis is at {y}"), 
            Point {x, y} => println!("On neither axis: ({x}, {y})"), 
        }

        // Destructuring enums 
        enum Message {
            Quit, 
            Move {x: i32, y: i32}, 
            Write(String), 
            ChangeColor(i32, i32, i32), 
        }

        let msg = Message::ChangeColor(0, 160, 255); 
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to restructure");
            }
            Message::Move{x, y} => {
                println!("Move in the x direction {x} and in the y direction {y}"); 
            }
            Message::Write(text) => {
                println!("Text message: {text}"); 
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}",)
            }
        }

        // Destructuring nested structs and enums 
        enum Color {
            Rgb (i32, i32, i32), 
            Hsv (i32, i32, i32), 
        }

        enum MessageTypeTwo {
            Quit, 
            Move {x: i32, y: i32}, 
            Write(String), 
            ChangeColor(Color), 
        }

        let msg = MessageTypeTwo::ChangeColor(Color::Hsv(0, 160, 255)); 
        match msg {
            MessageTypeTwo::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}"); 
            }
            MessageTypeTwo::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (), 
        }

        // Destructuring structs and tuples 
        let ((feet, inches), Point{x, y}) = ((3, 10), Point{x: 3, y: -10}); 

        // Ignoring values in a pattern: 
        // (a) ignore entire value with _ 
        fn foo(_:i32, y: i32) {
            println!("The code only uses the y parameter: {}", y); 
        }
        foo(3, 4); 

        // (b) ignore parts of a value with nested _ 
        let mut setting_value = Some(5); 
        let new_setting_value = Some(10); 

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value"); 
            }
            _ => {
                setting_value = new_setting_value; 
            }
        }
        println!("Setting is {:?}", setting_value); 

        let numbers = (2, 4, 8, 16, 32); 
        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}"); 
            }
        }

        // (c) ignoring unused variable by starting its name with _ 
        let _x = 5; // -> no warning 
        let y = 10;  // -> warning 

        let s = Some(String::from("Hello!")); 
        if let Some(_) = s {
            println!("Found a string"); 
        }
        println!("{:?}", s); 

        // (d) ignoring remaining parts of a value with .. 
        // .. with struct 
        struct Point3D {
            x: i32, 
            y: i32, 
            z: i32, 
        }
        let origin = Point3D {x: 0, y: 0, z: 0}; 
        match origin {
            Point3D {x, ..} => println!("x is {}", x), 
        }

        // .. with a tuple 
        let numbers = (2 ,4 ,8, 16, 32); 
        match numbers {
            (first, .., last) => {
                println!("First and last numbers: {first}, {last}"); 
            }
        }
        // but: (.., second, ..) is ambiguous and will generate a compile error 

        // Extra conditionals with match guard 
        // match guard: extra if condition 
        let num = Some(4); 
        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x), 
            Some(x) => println!("The number {} is odd", x), 
            None => (), 
        }


        // 
        let x = Some(5); 
        let y = 10; 
        match x {
            Some(50) => println!("Got 50"), 
            Some(n) if n == y => println!("Matched, n = {n}"), 
            _ => println!("Default case, x = {:?}", x), 
        }
        println!("at the end: x = {:?}, y = {y}", x); 

        // with || operator for multiple patterns 
        let x = 4; 
        let y = false; 
        match x {
            4 | 5 | 6 if y => println!("yes"),  // y is false: go to next arm 
            _ => println!("no"), 
        }

        // @ bindings 
        enum MessageTwo {
            Hello{ id: i32}, 
        }
        let msg = MessageTwo::Hello{id: 5}; 
        match msg {
            MessageTwo::Hello {
                id: id_variable @ 3..=7, 
            } => println!("Found an id in range, {}", id_variable), 
            MessageTwo::Hello {id: 10..=12} => {
                println!("Found an id in another range"); 
            }
            MessageTwo::Hello {id} => println!("Found some other id: {}", id), 
        }
    }

}
