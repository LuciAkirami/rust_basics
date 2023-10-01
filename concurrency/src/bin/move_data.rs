use std::thread;
fn main(){
    let v = vec![1,2,3,4];

    // here are moving var "v" to the spawned thread from main thread
    // to move data from main thread to child thread, you need to use the "move" word
    let handle = thread::spawn(move ||{
        println!("Variable Owned by Spawned Thread v: {v:?}");
    });
    // "v" is no longer valid in the main thread
    // println!("{v:?}"); error because value was "moved before"

    // to make sure our spawned thread is run before the main thread terminates the program
    handle.join().unwrap();
}