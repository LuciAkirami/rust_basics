// this enum can be used to create linked lists, Cons is a recursive type
// that can store values of different types
enum  List{
    Cons(i32,Box<List>),
    Nil
}

// using the Cons and Nil from List Enum
use crate::List::{Cons,Nil};

#[allow(unused_variables)]
fn main(){
    // a -> 4 -> 5
    let a = Cons(4,Box::new(Cons(5,Box::new(Nil))));
    // b -> 1 -> a
    let b = Cons(1,Box::new(a));
    // c -> 7 -> a
    // let c = Cons(7,Box::new(a)); error because the value is moved above
}