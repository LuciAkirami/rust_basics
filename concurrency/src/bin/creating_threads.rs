use std::thread;
use std::time::Duration;

fn main(){
    // ------------ Thread Basics - Spawning Threads ------------
    // Spawning a new thread, inside it running the for loop from 1 to 10
    thread::spawn(|| {
        for i in 1..10{
            println!("I'm from Spawned Thread, Value of i: {i}");
            thread::sleep(Duration::from_millis(1));
            // when line 9 executes, the processor "may move" from spawned thread to
            // main thread as the spawned thread is sleeping
        }
    });

    // This is the parent/main thread
    for i in 1..5{
        println!("I'm from Main Thread, Value of i: {i}");
        thread::sleep(Duration::from_millis(1));
        // when line 19 executes, the processor "may move" from main thread to
        // spawned thread as the main thread is sleeping, thus chaning threads
    }

    // The threads will probably take turns, but that isn’t 
    // guaranteed: it depends on how your operating system schedules the threads.
    // hence output might be different everytime u run it

    /* ------------- IMPORTANT NOTE --------------
    From output, we notice that, the for loop in the spawned thread did not run fully.
    This is because, the program exits when the main/parent thread finishes, thus when
    main thread finished 5 loops, the program finishes and all spawned threads are shutdown
    */

    
}