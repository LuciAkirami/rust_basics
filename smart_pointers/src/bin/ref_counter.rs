// ----------- Rc<T> (Reference Counter) ------------
// in main.rs, we have seen that the List "a" cannot be assigned to different values
// hence we have a smart pointer called Rc<T>, which is called the reference counter
// this smart pointer allows a variable to have multiple references and it keeps the
// count of them

// replaceing Box<T> with Rc<T>
enum  List{
    Cons(i32,Rc<List>),
    Nil
}

// using the Cons and Nil from List Enum
use crate::List::{Cons,Nil};
use std::rc::Rc;

#[allow(unused)]
fn main(){
    // a -> 4 -> 5
    let a = Rc::new(Cons(4,Rc::new(Cons(5,Rc::new(Nil)))));
    // checking the number of reference count to a
    println!("Num of references initially is {}",Rc::strong_count(&a));
    // b -> 1 -> a
    let b = Cons(1,Rc::clone(&a));
    println!("Num of references after assigning to 'b' are {}",Rc::strong_count(&a));
    // c -> 7 -> a
    let c = Cons(7,Rc::clone(&a));
    println!("Num of references after assigning to 'c' are {}",Rc::strong_count(&a));
    // droping c
    drop(c);
    println!("Num of references after dropping 'c' are {}",Rc::strong_count(&a));
}

/*
-> Referene Counter drops 'a' from the memory only after all it's references are dropped,
i.e. when the count of 'a' reaches to 1, and no other variable is referencing, then
at the end of main, even 'a' goes out out of scope, thus count reaches 0 and var dropped

-> Rc::clone() doesn't perform a deep copy like most types’ implementations of "clone" do 
Instead the call to Rc::clone only increments the reference count, which doesn’t 
take much time

-> Thus Rc<T> allows u to have multiple "immutable" references

-> Works only in single threaded scenarios
*/