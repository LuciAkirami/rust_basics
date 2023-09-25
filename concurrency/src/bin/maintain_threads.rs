use std::thread;
use std::time::Duration;

fn main(){
    // ----------- Waiting for all Threads to Complete with Join Handles -----------
    // The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, 
    // when we call the join method on it, will wait for its thread to finish.
    // "Blocking" a thread means that thread is prevented from performing work or exiting

    // here the spawn thread returns a JoinHandle type that's saved in the var "handle"
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // calling the .join() func will block the current thread running(main thread) until the
    // thread represented by the "handle" i.e "spawned thread" finishes/terminates
    // hence when we run the below command, first above loop executes then below main 
    // thread loop executes 
    // handle.join().unwrap();

    // Note: "Blocking" a thread means that thread is prevented from performing work or exiting

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // when the above for loop(main thread) finishes, the line 35 will run, when this line
    // gets executed, it will block the called thread, i.e. the main thread and wait until
    // the thread represented by the "handle"("spawned thread") until finishes/terminates 
    // and the the main thread will terminate the program
    handle.join().unwrap();

}