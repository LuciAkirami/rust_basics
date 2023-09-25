use std::thread;
fn main(){
    let v = vec![1,2,3,4];

    // here are moving var "v" to the spawned thread from main thread
    let handle = thread::spawn(move ||{
        println!("Variable Owned by Spawned Thread v: {v:?}");
    });
    // "v" is no longer valid in the main thread

    // to make sure our spawned thread is run before the main thread terminates the program
    handle.join().unwrap();
}