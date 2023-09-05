// -------- STACK MEMORY --------
// usually all primitive data types are stored in stack
// these include integers, floats, bool, arrays
// elements can be fastly accessed from stack
// copying elements is also fast in stack memory
// A stack frame is a mapping from variables to values within a single scope, such as a function
// The stack holds data associated with a specific function like main, 
// and data is delocated when function ends/returns

// --------- HEAP MEMORY ------------
// heap is also a memory where non primitive types are stored
// these include vectors, Strings, etc
// Pointers are used to point the location of data in Heaps
// accessing elements in heap or copying is expensive 
// and slow compared to stack
// the heap memory is delocated when the variable holding the heap is delocated

#[allow(unused_variables)]
fn main() {
    // here x and y are stored in stack -> primitive types
    // the value of x is copied to y, as copying is inexpensive in stack
    let x = 10;
    let y = x;

    // but what if there is a large value, like an array with million elements
    let arr1 = [0,1_000_000]; 
    let arr2 = arr1; 
    // now our stack has 2 million elements, and this copy operation is
    // expensive as we are copying 1 million elements and now we have
    // to store 2 million elements
   
    // ---- Solution HEAPS --------
    // instead of copying, we can transfer the access to data from arr1 to arr2 using pointers
    // Rust provides a construct "Box" for putting data on heap
    let a = Box::new([0,1_00]); // the variable "a" points to the data on heap
    let b = a; 
    // now the data is transfered from "a" to "b" => "a is moved to b"
    // meaning "b" points to the data now on heap and it cannot be accessed by "a"

    // ----- Understanding Ownership --------
    // here "a" is bound to Box::new([0,1_00]) => "a" 'owns' Box::new([0,1_00])
    let a = Box::new([0,1_00]);
    
    // now let's assign "a" to another variable
    let b = a; // this "moves" the "ownership" from "a" to "b"
    // now the "b" is the "owns" the Box::new([0,1_00]) and "a" is no longer the "owner"
    
    // ---- Variables Cannot Be Used after Moved ----
    // the variable "a" doesn't work as its ownership is "moved"
    // println!("{:?}",a) -> throws a error saying a is moved
    // Moving ownership of heap data avoids undefined behavior from reading deallocated memory.

    // ---- Box Delocation Principle ------
    // as "b" holds the data, when the main function ends, the variable "b" is out of
    // scope and is delocated and thus the heap will be delocated

    // ---- Collections ----
    // collects like String, Vector, HashMaps as they are not fix sized, stoed in heap
    let example_string = String::from("Hello") ;
    // "Hello" is stored in heap and example_string, the pointer that points to Hello 
    // is stored in a stack / stack frame / variable frame

    // ---- Cloning is Not Moving -----
    // when you want to copy a heap to another var instead of move, you use Clone
    let my_string = String::from("World!");
    let my_string_2 = my_string.clone();
    // here my_string is not being "moved", that is the owner is not changing here
    // instead we are creating a clone of the data in my_string and assinging this
    // cloned data to a new pointer my_string_2, these two var now point 
    // to "Hello" at two different locations, i.e. two "Hello" exist in Heap
    // as we are not "moving" the data here, hence we can reuse the var my_string later
    // println!("{:?}",my_string) -> successfully runs

    // --- Moving Data to Functions ---
    let my_string = String::from("World!");
    take_ownership(my_string);
    // println!("{:?}",my_string) -> doesn't work as the ownership is moved to "word"
 
}

fn take_ownership(word: String){
    println!("The ownership belongs to 'word' and it holds {:?}",word);
    // after this fn ends, the "word" var is delocated, hence the heap also delocates
}

/*
Ownership is primarily a discipline of heap management:
-> All heap data must be owned by exactly one variable.
-> Rust deallocates heap data once its owner goes out of scope.
-> Ownership can be transferred by moves, which happen on assignments and function calls.
-> Heap data can only be accessed through its current owner, not a previous owner.
*/