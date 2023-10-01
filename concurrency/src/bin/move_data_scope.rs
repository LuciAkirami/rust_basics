// moving data into the spawned thread without giving away the ownership
// referencing data from main thread to spawned thread
#![allow(unused)]
use std::thread;
use std::time::Duration;

struct Person{
    name: String,
    age: u32
}

fn main(){
    let person1 = Person{name:"Lucifer".to_string(),age:20};

    // defining a closure that takes reference to the person
    let print_name = || {
        println!("Printing from Spawned Thread {:?}",&person1.name);
    };

    thread::scope(|scope| {
        scope.spawn(print_name); // .join() will be automatically called
    });

    println!("Printing from Main Thread {:?}",person1.name);
}