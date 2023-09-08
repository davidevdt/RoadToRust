mod exercises; 
use exercises::run; 

fn main() {
    // Vectors 
    {
        // Create New Vectors 
        let v: Vec<i32> = Vec::new(); 

        // With values 1, 2, 3 (default type: i32)
        let v = vec![1, 2, 3]; 

        // With push 
        let mut v = Vec::new(); 
        v.push(5); 
        v.push(6); 
        v.push(7); 
        v.push(8); 
    }

    {
        // Reading elements in vectors 
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];

        // Modify element of a vector 
        // let mut v = vec![1, 2, 3, 4, 5];
        // let mut third: &mut i32 = &mut v[2];
        // *third += 5; // -> this will modify the third element of a vector  
        println!("The third element is {third}"); 

        let third: Option<&i32> = v.get(2); // If index is outside the vector, it returns None 
        match third {
            Some(third) => println!("The third element is {third}"), 
            None => println!("There is no thurd element."), 
        } 
    }

    {
        // Borrowing rules with vectors 
        // The following code won't compile, as push might require copying (also first) to a new location in address
        // let mut v = vec![1, 2, 3, 4, 5];
        // let first = &v[0]; // -> with this uncommented, it won't compile 
        // v.push(6);
        // println!("The first element is: {first}");
    }

    {
        // Iterating over values of a vector 
        let v = vec![100, 32, 57]; 
        for i in &v {
            println!("{i}")
        }

        // Mutating vector objects 
        let mut v = vec![100, 32, 57]; 
        for i in &mut v {
            *i += 50; 
        }
    }

    {
        // Enum's to store multiple types 
        enum SpreadSheetCell {
            Int(i32), 
            Float(f64), 
            Text(String), 
        }

        let row = vec![
            SpreadSheetCell::Int(3), 
            SpreadSheetCell::Text(String::from("blue")), 
            SpreadSheetCell::Float(10.12), 
        ]; 
    }

    {
        // Dropping a vector drops its elements 
        let v = vec![1, 2, 3, 4]; 
        // do stuff with v 
    } // <- v goes out of scope and is freed here 

    // Strings 
    {
        // Creating a new string 
        let mut s = String::new(); 

        // Creating s string from literal
        let data = "initial contents"; 
        let s = data.to_string(); 
        // directly from literal: 
        let s = "initial contents".to_string(); 
    }

    {
        // Strings from different languages (UTF-8 encode)
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    {
        // Updating a string 
        let mut s = String::from("foo"); 
        s.push_str("bar"); // s = foobar 

        // push_str takes a string slice as input 
        let mut s1 = String::from("foo"); 
        let s2 = "bar"; 
        s1.push_str(s2); 
        println!("s2 is {s2}"); // -> If push_str took owenrship of s2 we woulnd't be able to see this 

        // Push takes only one character as an input 
        let mut s = String::from("lo"); 
        s.push('l'); // -> s = "lol"

        // Combining two strings 
        let s1 = String::from("Hello, "); 
        let s2 = String::from("world!"); 
        let s3 = s1 + &s2; // note: s1 moved here and can no longer be used 
        // fn add(self, s: &str) -> String  // signature of generic add with concrete str // only String + &str

        // Adding multiple strings 
        let s1 = String::from("tic"); 
        let s2 = String::from("tac"); 
        let s3 = String::from("toe"); 
        // let s = s1 + "-" + &s2 + "-" + &s3;
        let s = format!("{s1}-{s2}-{s3}"); 
        println!("{s}");
    }

    {
        // Indexing strings 
        // This will throw an error: 
        // let s1 = String::from("Hello"); 
        // let h = s1[0]; 

        // Note: String is a wrapper around Vec<u8> 

        // Indexing with a range 
        let hello = "Здравствуйте";
        let s = &hello[0..4];   // -> It takes the first four bytes of the string 
        println!{"{s}"};        // -> So this prints Зд
        // &hello[0..1] // -> this will make Rust panic (invalid byte indexing) 
    }

    {
        // Iterating over strings 

        // Iterating over characters 
        for c in "Зд".chars() {
            println!("{c}"); 
        }

        // Iterating over bytes 
        for b in "Зд".bytes() {
            println!("{b}"); 
        }
    }

    // Hash maps

    {
        // Creating an hash map 
        use std::collections::HashMap; 

        let mut scores = HashMap::new(); 
        scores.insert(String::from("Blue"), 10); 
        scores.insert(String::from("Yellow"), 50); 
    }

    {
        // Accessing values from the map 
        use std::collections::HashMap; // HashMap<K, V>
        let mut scores = HashMap::new(); 
        scores.insert(String::from("Blue"), 10); 
        scores.insert(String::from("Yellow"), 50); 
        let team_name = String::from("Blue"); 
        let score = scores.get(&team_name).copied().unwrap_or(0); // -> get returns Option<&V>
                                                                  // -> copied() to get Option<i32> rather than Option<&i32>
                                                                  // -> unwrap_or(0) sets score to 0 if key doesn't have a value
    }

    {
        // Iterate over key/value pair with for loop 
        use std::collections::HashMap; 
        let mut scores = HashMap::new(); 
        scores.insert(String::from("Blue"), 10); 
        scores.insert(String::from("Yellow"), 50); 

        for (key, value) in &scores {
            println!("{key}: {value}"); 
        }
    }

    {
        // Hash maps and ownership 
        // For types that implement the Copy trait (like i32) values are copied in hash map 
        // For owned values like String, the values will be moved and hash map will become owner 
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new(); 
        map.insert(field_name, field_value); 
        // field_name and field_value are invalid at this point
    }

    {
        // Updating a hash map 

        // Overwriting a value 
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);

        // Adding a key and a value only if the key isn't present 
        let mut scores = HashMap::new(); 
        scores.insert(String::from("Blue"), 10); 
        scores.entry(String::from("Yellow")).or_insert(50); // -> or_insert: returns a mutable reference to the value if key exists,
        scores.entry(String::from("Blue")).or_insert(50);  // -> or inserts the parameter as the value for the keys if key does not exist
        println!("{:?}", scores); 

        // Update  value based on old value 
        let text = "hellow world wonderful world"; 
        let mut map = HashMap::new(); 
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0); // inserts zero if the key does wnot exist in the map 
            *count += 1; 
        }
        println!("{:?}", map);
    }

    {
        use std::io; 
        println!("Run exercises? y/n"); 
        let mut choice = String::new(); 
        io::stdin().read_line(&mut choice).expect("Could not read input"); 
        if choice.trim() == "Y" || choice.trim() == "y" {
            run(); 
        } 
    }
}
