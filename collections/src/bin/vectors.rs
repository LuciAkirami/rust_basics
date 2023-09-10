#![allow(unused_variables,unused_mut)]
fn main(){
    // creating a vector
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];

    // accesing elements of vector
    let ele1 = &vec2[1];
    // &vec2[1] will give us a reference to that ele at that index value
    // let ele = vec2[1]; a "move" occures here and we can no longer use vec2

    println!("Elements in the Vector");
    for ele in &vec2{ // important to use &vec2, because we are borrowing it
        println!("{}",ele);
    }
    // if we use "vec2" instead "&vec2", then "vec2" will be moved when for loop 
    // is run and after the for loop we cannot use the "vec2 anymore"

    // mutable reference to vector
    println!("\nBefore for loop: {:?}",vec2);
    for ele in &mut vec2{ 
        *ele+=1; // all elements in the vec2 are updated by 1
    }
    println!("After for loop: {:?}\n",vec2);

    // accesing elements with .get()
    let ele1 = vec2.get(0); // get returns an Option, so we need to unwrap()
    println!("First element in the Vector: {}\n",ele1.unwrap());
    // this code fails if we give an index where element is not present

    // more safe code
    let ele1 = vec2.get(2);
    match ele1{
        Some(i) => println!("Element in the Vector is: {}\n",i),
        None =>  println!("There is no element at that index\n"),
    }
    // this code works even if the element is not present at that index

    // updating / pushing / poping elements from the vector
    // we can only do these changes when the vector is mutable
    let mut vec3 = vec![1,2,3,4];
    println!("Elements in the Vector: {:?}",vec3);
    vec3[3] = 10;
    vec3.push(20);
    println!("Elements in the Vector After Pushing: {:?}",vec3);
    vec3.pop(); // pop() removes and returns the last element from vec
    println!("Elements in the Vector After Popping: {:?}",vec3);

    // Working with vector of Strings
    let v = vec![String::from("Hello ")];
    // let mut s = v[0]; // here a "move" operation is taking place
    // but this results in an error because, here we are trying to move "String"
    // and "String" cannot be moved as it doesn't have "Copy" trait
    // the above code works with other data types like int and float

    // another example
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0]; // here we are copying &String that doesn't down heap data
    // let s: String = *s_ref; this fails
    // The issue is that the vector v owns the string "Hello world". 
    // When we dereference s_ref, that tries to take ownership of the string 
    // from the vector. But references are non-owning pointers — we can't take 
    // ownership through a reference. Therefore Rust complains that we 
    // "cannot move out of [...] a shared reference".

    /*
    In sum, if a value does not own heap data, then it can be copied without a move. 
    For example:
        An i32 does not own heap data, so it can be copied without a move.
        A String does own heap data, so it can not be copied without a move.
        An &String does not own heap data, so it can be copied without a move.
    */

    let mut x = vec![1,2];

    // using iter()
    let mut v         = vec![1, 2];
    let mut iter = v.iter(); // v.iter() takes in a reference of v
    // so an immutable borrow occurs here
    // iter().next() returns an Option<&T>
    // let vec6 = &mut v;this mutable borrow fails as the iter() (immutble borrow)
    // is being used below
    let n1: &i32                = iter.next().unwrap();
    let n2: &i32                = iter.next().unwrap();
    let end: Option<&i32>       = iter.next();

    // length of the vector
    let length = v.len(); // returns the length of the vector


}

