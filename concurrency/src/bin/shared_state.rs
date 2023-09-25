// ---- Multiple Threads using Same Shared Memory ----
#![allow(unused_imports)]
use std::thread;
use std::sync::mpsc; 
use std::time::Duration;
use std::sync::{Arc,Mutex};

/*
Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread 
to access some data at any given time. To access the data in a mutex, a thread must 
first signal that it wants access by asking to acquire the mutex’s lock. The lock is a
data structure that is part of the mutex that keeps track of who currently has exclusive 
access to the data. Therefore, the mutex is described as guarding the data it holds via 
the locking system.

Mutexes have a reputation for being difficult to use because you have to remember two rules:

1. You must attempt to acquire the lock before using the data.
2. When you’re done with the data that the mutex guards, you must unlock the data so other 
threads can acquire the lock.
*/
fn main(){
    // creating a new Mutex using the fn "new"
    let m = Mutex::new(5);

    { // Here this scope takes control of the mutex from the main scope by blocking it,
        //  by calling the lock() method, thus this scope can access and modify it.

        // lock() - Acquires a mutex, blocking the current thread until it is able to do so.
        // the lock() returns a LockResult<MutexGuard<'_, T>> which is unwrapped
        let mut num = m.lock().unwrap();
        // modifying the value inside mutex
        *num = 6;

        // MutexGuard smart pointer implements Deref to point at our inner data; 
        // the smart pointer also has a Drop implementation that releases the lock 
        // automatically when a MutexGuard goes out of scope, which happens at the end of 
        // the inner scope

        // after this scope ends, the mutex is given back to the main scope, so the main
        // scope now owns the mutex "m" and can change its data
    }
    // the same scope example fits with threads. When a thread owns the mutex with mutex.lock(),
    // this will block the current thread so it can't do any worj until its our turn to have lock

    // After dropping the lock, we can print the mutex value and see that we were able to change 
    // the inner i32 to 6.
    println!("m = {:?}", m);


    // ------ Sharing Mutex Between Multiple Threads -------
    /*
    We create a counter variable to hold an i32 inside a Mutex<T> inside an Arc<T>. 
    Next, we create 10 threads by iterating over a range of numbers. We use thread::spawn 
    and give all the threads the same closure: one that moves the counter into the thread, 
    acquires a lock on the Mutex<T> by calling the lock method, and then adds 1 to the value 
    in the mutex. When a thread finishes running its closure, num will go out of scope and 
    release the lock so another thread can acquire it.

    Here Arc<T>(Atomic Reference Counting) is a type similar to Rc<T>, that is safe to use
    in concurrent situations. In essence, Arc<T> is a type that lets us share ownership of data 
    across threads safely.
    */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
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