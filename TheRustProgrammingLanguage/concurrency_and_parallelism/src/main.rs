fn main() {
    
    // Threads 
    {
        // Creating a new thread with spawn 
        // -> The following snippet will show that once it's over, the main thread won't wait for spawned unfinished threads
        use std::thread; 
        use std::time::Duration; 

        // thread::spawn(|| {
        //     for i in 1..10 {
        //         println!("hi number {} from the spawned thread!", i); 
        //         thread::sleep(Duration::from_millis(1)); 
        //     }
        // }); 

        // for i in 1..5 {
        //     println!("hi number {} from the main thread!", i); 
        //     thread::sleep(Duration::from_millis(1)); 
        // }

        // thread::sleep -> stops execution for a short duration 
        // the return type of spwan is JoinHandle 

        {
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned thread!", i); 
                    thread::sleep(Duration::from_millis(1)); 
                }
            }); 

            // handle.join().unwrap()   // -> this will block until the spawned thread ends 

            for i in 1..5 {
                println!("hi number {} from the main thread!", i); 
                thread::sleep(Duration::from_millis(1)); 
            }

            handle.join().unwrap(); 

        }

        // Using move closures with threads 
        {
            let v = vec![1, 2, 3]; 
            let handle = thread::spawn(move || {            // -> the move keyword is essential to have the closure take ownership of v 
                println!("Here's a vector: {:?}", v);   
            }); 

            // drop(v)  // -> if we were to use move in the closure, this wouldn't compile 

            handle.join().unwrap(); 
        }
    }   

    // Using message passing to transfer data between threads 
    {
        // Channel: programming concept in which data is sent from one thread to another 

        use std::sync::mpsc; 
        use std::thread; 

        let (tx, rx) = mpsc::channel(); // multiple producers - single consumer [transmitter and receiver ends]

        thread::spawn(move || {
            let val = String::from("hi"); 
            tx.send(val).unwrap(); 
        }); 

        let received = rx.recv().unwrap();  // -> recv blocks execution until it receives something and returns Result<T,E>
                                            // -> try_recv doesn't block but returns a Result<T,E> immediately
        println!("Got: {}", received); 

        // Channels and ownership transference 
        let (tx, rx) = mpsc::channel(); 

        thread::spawn(move || {
            let val = String::from("hi"); 
            tx.send(val).unwrap(); 
            // println!("val is {}", val);  // -> would not compile: val is owned by another thread  
        }); 

        let received = rx.recv().unwrap(); 
        println!("Got: {}", received); 

        // Sending multiple values and seeing the receiver waiting 
        use std::time::Duration; 

        let (tx, rx) = mpsc::channel(); 
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"), 
                String::from("from"), 
                String::from("the"), 
                String::from("thread"), 
            ]; 

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1)); 
            }
        }); 

        for received in rx {
            println!("Got: {}", received); 
        }

        // Creating multiple producers by cloning the transmitter 
        let (tx, rx) = mpsc::channel(); 
        let tx1 = tx.clone(); 

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap(); 
                thread::sleep(Duration::from_secs(1)); 
            }
        }); 

        thread::spawn(move || {
            let vals = vec![
                String::from("more"), 
                String::from("messages"), 
                String::from("for"), 
                String::from("you"),
            ]; 

            for val in vals {
                tx.send(val).unwrap(); 
                thread::sleep(Duration::from_secs(1));
            }
        }); 

        for received in rx {
            println!("Got: {}", received); 
        }
    }

    // Shared-state concurrency 
    {
        // Mutexes to allow access data one thread at a time 
        // Mutex<T> // -> smart pointer; implements Drop 
        use std::sync::Mutex; 

        let m = Mutex::new(5);  

        {
            let mut num = m.lock().unwrap();  // -> unwrap: if another thread holding the lock panics, this call fails
                                              // -> and so this thread would panic if we were in that situation 
                                              // -> The type of m is Mutex<i32> so we need to use lock()  
            *num = 6; 
        }

        println!("m = {:?}", m); 

        // Multi-threaded shared counter and Atomic Reference Counter Arc<T> 
        use std::thread; 
        use std::sync::Arc;         // -> Arc a pointer like Rc safe across threads 

        let counter = Arc::new(Mutex::new(0)); 
        let mut handles = vec![]; 

        for _ in 0..10 {
            let counter = Arc::clone(&counter); 
            let handle  = thread::spawn(move || {
                let mut num = counter.lock().unwrap(); 
                *num += 1; 
            }); 
            handles.push(handle); 
        }

        for handle in handles {
            handle.join().unwrap(); 
        } 

        println!("Result: {}", *counter.lock().unwrap()); 
    }

    // Send and sync traits 
    // Any type T is Sync if &T (immutable rereference to T) is Send 

}
