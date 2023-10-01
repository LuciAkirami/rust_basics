// Import necessary modules from the standard library.
#![allow(unused_variables)] // Allow unused variables for demonstration purposes.
use std::thread::{scope, sleep}; // Import thread-related modules.
use std::time::Duration; // Import the Duration type.
use std::sync::Mutex; // Import the Mutex type for synchronization.

fn main() {
    // Create a Mutex (mutual exclusion) to safely share data across threads.
    // Initialize it with a string "Lucifer".
    let name = Mutex::new("Lucifer".to_string());

    // Define a closure "change_name" to be executed by a spawned thread.
    let change_name = || {
        // Sleep for 2 seconds (simulating some time-consuming task).
        sleep(Duration::from_secs(2));
        // Acquire a lock on the Mutex, ensuring exclusive access.
        let mut named = name.lock().unwrap();
        // Modify the data inside the Mutex by appending " How are you?" to the string.
        named.push_str(" How are you?");
    };

    // Define another closure "change_name2" to be executed by another spawned thread.
    let change_name2 = || {
        // Acquire a lock on the Mutex to ensure exclusive access.
        let mut named2 = name.lock().unwrap();
        // Modify the data inside the Mutex by appending ". I'm fine" to the string.
        named2.push_str(". I'm fine");
    };

    // Create a new thread scope to manage spawned threads.
    scope(|scope| {
        // Spawn the "change_name" closure in a new thread within the scope.
        scope.spawn(change_name);
        // Spawn the "change_name2" closure in another new thread within the same scope.
        scope.spawn(change_name2);
    });

    // Acquire a lock on the Mutex to access the modified data.
    let changed_data = name.lock().unwrap();

    // Print the modified data (the string inside the Mutex).
    println!("{changed_data:?}");
}

/*
Here "change_name2" will be running first and I'm fine will get added first,
irrespective of which ever spawned thread runs first. 

Let's assume the the thread containing "change_name" will run first. When this is
run, the code inside the closure gets executed and it has to sleep for 2 seconds
before aquiring the lock. In these 2 locks the other thread containing "change_name2"
will acquire the lock to the "name" variable and append "I'm fine" first

If we remove the "sleep", then it's purely random, which ever of ("I'm fine",
"How are you?") can be added first.
*/