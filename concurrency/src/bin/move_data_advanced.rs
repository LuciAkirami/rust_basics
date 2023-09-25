use std::thread;
use std::sync::mpsc; 
use std::time::Duration;
// to create a channel for communication between threads, we use mpsc. For channel to
// work, both thread that's transimiting and thread that's receiving must be active
// else the channel is said to be closed if either one drops
fn main(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // using tx.send() to send a messagem send() returns a Result<T, E> 
        // using unwrap() to unwrap the OK() from Result else panics if any errors
        // usually returns error if the receiver is dropped
        let message = "Sending Message from Spawn thread";
        tx.send(message).unwrap(); // send() takes the ownership of "message"
        // println!("{message:?}") no longer valid as it's "moved" in above statement
    });

    // using rx.recv() to receive the message
    // Note: Using rx.recv() will block the main thread until it receives the message
    let k = rx.recv().unwrap();
    // the ownership of "message" from spawned thread is "moved" to "k"

    // The try_recv method doesn’t block, but will instead return a Result<T, E> 
    // immediately: an Ok value holding a message if one is available and an Err value 
    // if there aren’t any messages this time. Using try_recv is useful if this thread 
    // has other work to do while waiting for messages
    // let k = rx.try_recv().unwrap();

    println!("Messaged Received: {k:?}");


    // Multiple Producers Example
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // cloning tx to create another transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            // waiting a second to send another message
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

    // we can use for loop to iterate over "rx"
    for received in rx {
        println!("Got: {}", received);
    }
    // the ordering of messages is non deterministic
}