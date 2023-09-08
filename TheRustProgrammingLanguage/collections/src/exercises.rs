pub fn run() {
    // Exercises
    {
        println!("--- Exercise 1 - Given list of integers, return median and mode of a vector ---"); 
        // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
        // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
        use std::io; 

        fn calculate_median(sorted_vec: &Vec<i32>) -> f64 {

            let n: usize = sorted_vec.len(); 
            let is_even_len: bool = match n % 2 {
                0 => true,
                _ => false, 
            }; 

            if !is_even_len {   // Case 1: odd number of elements 
                let middle_idx = f64::from(n as u32) / 2.0 - 0.5; 
                let middle_idx = middle_idx as usize; 
                let median = sorted_vec.get(middle_idx); 
                f64::from(*median.unwrap()) 
            } else {    // Case 2: even number of elements
                let middle_idx_left = (n / 2) - 1;
                let middle_idx_right =  n / 2; 
                let middle_idx_left = middle_idx_left as usize; 
                let middle_idx_right = middle_idx_right as usize; 
                let left_median = sorted_vec.get(middle_idx_left); 
                let right_median = sorted_vec.get(middle_idx_right); 
                f64::from(left_median.unwrap() + right_median.unwrap()) / 2.0
            }
        }

        fn calculate_mode(vec: &Vec<i32>) -> Vec<i32> {
            use std::collections::HashMap;
            let mut count_map = HashMap::new();
            let mut mode: Vec<i32> = Vec::new();
            let mut max_count: u32 = 0;   

            for (i, &val) in vec.iter().enumerate() {
                let count = count_map.entry(val).or_insert(0);
                *count += 1; 
            }

            for (val, count) in &count_map {
                if count < &max_count {continue}
                else if count == &max_count {
                    mode.push(*val); 
                } else {
                    max_count = *count ; 
                    mode.clear();
                    mode.push(*val);  
                }
            }
            mode
        }

        fn calculate_median_and_mode(input_vec: &mut Vec<i32>) -> (f64, Vec<i32>) {

            input_vec.sort(); 
            let median = calculate_median(&input_vec); 
            let mode = calculate_mode(&input_vec); 
            return (median, mode); 
        }

        // Run the program 
        println!("How many integers do you want to input?"); 
        let mut n = String::new(); 
        io::stdin().read_line(&mut n).expect("Error reading input"); 
        let n: usize = n.trim().parse().expect("Failed to convert n"); 
        let mut input_vec = Vec::with_capacity(n);   

        for i in 0..n {
            let mut input = String::new();  
            io::stdin().read_line(&mut input).expect("Error reading input"); 
            input_vec.push(input.trim().parse().expect("Failed to convert the input")); 
        }

        let (median, mode) = calculate_median_and_mode(&mut input_vec); 
        println!("The median is {}", median); 
        print!("The mode is ");  
        for (i, &m) in mode.iter().enumerate() {
            print!("{}", m); 
            if mode.len() > 1 && i != mode.len()-1 {
                print!(", ")
            }
        }
        print!("\n"); 
        println!("{:?}", input_vec); 

        // Tests
        let v1 = vec![1, 2, 3, 4, 5]; 
        let v2 = vec![1, 2, 3, 4, 5, 6, 7, 8]; 
        assert_eq!(calculate_median(&v1), 3.0);  
        assert_eq!(calculate_median(&v2), 4.5); 

        let v3 = vec![4, 5, 6, 7, 8, 8]; 
        let v4 = vec![9, 9, 10, 10, 10, 11, 11, 11, 12]; 
        let mut mode1 = calculate_mode(&v1); 
        let mut mode3 = calculate_mode(&v3); 
        let mut mode4 = calculate_mode(&v4);
        mode1.sort(); 
        mode3.sort(); 
        mode4.sort(); 
        assert_eq!(mode1, [1, 2, 3, 4, 5]); 
        assert_eq!(mode3, [8]); 
        assert_eq!(mode4, [10, 11]); 
    }

    {
        println!("--- Exercise 2 - Convert string to pig latin ---"); 
        // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, 
        // so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead 
        // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
        use std::io;         

        // const consonants: [char; 20] = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 't', 'v', 'w', 'x', 'y', 'z'];
        const vocals: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        fn starts_with_vocal(string_to_convert: &str) -> bool {
            let starting_char = string_to_convert.chars().next().unwrap(); 
            // if vocals.contains(&starting_char.to_lowercase().next().unwrap()) {return true}
            if vocals.contains(&starting_char) {return true}
            return false 
        }

        fn convert_string(string_to_convert: &str) -> String {
            let mut converted_string = String::new(); 

            if starts_with_vocal(&string_to_convert) {
                converted_string = format!("{}-hay", &string_to_convert); 
            } else {
                converted_string = format!("{}-{}ay", &string_to_convert[1..], &string_to_convert[..1]); 
            }
            converted_string
        }

        // Run the program 
        let mut input_string = String::new(); 
        println!("Please write the string to convert:"); 
        io::stdin().read_line(&mut input_string).expect("Error reading input"); 
        input_string = input_string.replace("\n", ""); 
        input_string = input_string.to_lowercase(); 
        let s = convert_string(&input_string); 
        println!("{} becomes: {}", input_string, s); 
        // println!("{}", s); 

        // Tests 
        let word_1 = "mama"; 
        let word_2 = "aunt"; 
        let word_3 = "cousin"; 
        let word_4 = "aunts"; 
        assert_eq!(starts_with_vocal(&word_1), false, "Test1"); 
        assert_eq!(starts_with_vocal(&word_2), true, "Test2");
        assert_eq!(starts_with_vocal(&word_3), false, "Test3"); 
        assert_eq!(starts_with_vocal(&word_4), true, "Test4");  

        let word_5 = "first"; 
        let word_6 = "apple"; 
        assert_eq!(convert_string(&word_5), "irst-fay"); 
        assert_eq!(convert_string(&word_6), "apple-hay");    

    }

    {
        println!("--- Exercise 3 - Department employee manager ---");
        // Using a hash map and vectors, create a text interface to allow a user to add employee names 
        // to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
        // Then let the user retrieve a list of all people in a department or all people in the company by department, 
        // sorted alphabetically.

        use std::io; 
        use std::collections::HashMap; 
        let mut employees_map: HashMap<Department, Vec<Employee>> = HashMap::new();

        #[derive(Debug, Clone)]
        struct Employee {
            name: String, 
        } 

        #[derive(Debug, Eq, PartialEq, Hash, Clone)]
        struct Department {
            name: String, 
        }

        fn request_employee_and_department() -> Option<(Employee, Department)> {
            println!("Please add the desired command in this format:\n\
            Add (NAME) to (DEPARTMENT)"); 
            let mut add_input = String::new(); 
            io::stdin().read_line(&mut add_input).expect("Error reading input"); 

            add_input = add_input.trim().to_string(); 
            let mut department_name = String::new(); 
            let mut employee_name = String::new(); 
            let mut found_name: bool = false; 

            for (idx, word) in add_input.split_whitespace().enumerate()  {
                if idx == 0 && (word != "Add" && word != "add") {
                    println!("Invalid command; please start the request with \"Add\""); 
                    return None; 
                }
                
                if idx == 1 { 
                    employee_name.push_str(word); 
                }
                
                if idx > 1 {
                    if !found_name {
                        if !found_name {
                            if word == "to" || word == "To" || word == "TO" {
                                found_name = true; 
                                continue; 
                            } else {
                                employee_name.push_str(" "); 
                                employee_name.push_str(word); 
                            }
                        }
                    } else {
                        if department_name != "" {department_name.push_str(" ");}
                        department_name.push_str(word); 
                    }
                } 
            }
            // println!("{} in department {}", name, department); 
            let employee = Employee{name: employee_name}; 
            let department = Department{name: department_name}; 
            Some((employee, department))
        }

        fn add_employee_to_map(employee: &Employee, department: &Department, employees_map: &mut HashMap<Department, Vec<Employee>>) {
            let employee_list = employees_map.entry((*department).clone()).or_insert_with(Vec::new); 
            employee_list.push((*employee).clone()); 

            // let print_closure = || println!("{:?}", employees_map);
            // print_closure(); 
            // println!("{:?}", || employees_map); 
        }

        fn update_employee_list(employees_map: &mut HashMap<Department, Vec<Employee>>) {
            let added_input_attempt = request_employee_and_department(); 
            match added_input_attempt {
                None => (), 
                Some((employee, department)) => {
                    add_employee_to_map(&employee, &department, employees_map); 
                }
            }
        }

        fn display_employees(employees_map: &mut HashMap<Department, Vec<Employee>>) {
            println!("Please select department that you want to print");
            // for (dep, _emp) in employees_map {   // -> iterates over owned key-value pairs, the hashmap is being consumed (can't be used after the loop)
            // for dep in employees_map.keys() { // -> iterates over ref to hashmap's keys 
            // for (dep, emp_vec) in employees_map.iter() { // -> iterates over references to key-value pairs; the map can be reused after the loop 
            for (dep, emp_vec) in employees_map.iter_mut() { // -> iterates over mutable references to key-value pairs; the map can be reused after the loop 
                println!("{}", dep.name);
                emp_vec.sort_by(|a, b| a.name.cmp(&b.name));
            }
            println!("All (all departments)"); 

            let mut dep_to_lookup = String::new(); 
            io::stdin().read_line(&mut dep_to_lookup).expect("Could not read input"); 
            dep_to_lookup = dep_to_lookup.trim().to_string(); 

            if dep_to_lookup == "All" || dep_to_lookup == "all" {
                
                // Solution 1: iterate over a sorted clone of original vector 
                // for(dep, emp_vec) in employees_map {
                // for(dep, emp_vec) in employees_map.iter_mut() {
                //     println!("Department: {}", dep.name); 
                //     let mut emp_clone = emp_vec.clone(); 
                //     emp_clone.sort_by(|a, b| a.name.cmp(&b.name));
                //     for emp in emp_clone {
                //         println!("\tEmployee: {}", emp.name); 
                //     }
                // }
                
                // Solution 2: iterate over sorted original vector (already sorted previous so not needed )
                // for (dep, emp_vec) in employees_map {
                for (dep, emp_vec) in employees_map.iter() {
                // for(dep, emp_vec) in employees_map.iter_mut() {
                    println!("Department: {}", dep.name); 
                    // let mut emp_clone = emp_vec.clone(); 
                    // emp_vec.sort_by(|a, b| a.name.cmp(&b.name));
                    for emp in emp_vec {
                        println!("\tEmployee: {}", emp.name); 
                    }
                }
                return(); 
            } 

            let department_key = Department{name: dep_to_lookup}; 

            if let Some(emp_vec) = employees_map.get(&department_key) {
                println!("Department: {}", department_key.name);
                // let mut emp_clone = emp_vec.clone(); 
                // emp_clone.sort_by(|a, b| a.name.cmp(&b.name));
                // for emp in emp_clone {
                for emp in emp_vec {
                    println!("\tEmployee: {}", emp.name); 
                }
            } else {
                println!("Department not found.")
            }
        }

        loop {

            println!("\n\nPlease select one of the following options:\n\
            1. Add employee to department\n\
            2. Display employees\n\
            q. Quit"); 

            let mut user_input = String::new(); 
            io::stdin().read_line(&mut user_input).expect("Couldn't read input"); 

            match user_input.as_str().trim() {
                "1" => update_employee_list(&mut employees_map), 
                "2" => display_employees(&mut employees_map), 
                "q" => {break} , 
                _ => {
                    println!("Select option 1, 2, or q to quit");
                    continue
                }
            }
        }

        println!("Program terminated.")
    }
}