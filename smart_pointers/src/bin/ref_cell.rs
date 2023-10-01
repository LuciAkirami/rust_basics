/*
--------- Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T> --------------

A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have 
multiple owners of some data, but it only gives immutable access to that data. If you have 
an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and 
that you can mutate!

For example, recall the cons list example in Listing 15-18 where we used Rc<T> to allow 
multiple lists to share ownership of another list. Because Rc<T> holds only immutable values, 
we can’t change any of the values in the list once we’ve created them. Let’s add in RefCell<T> 
to gain the ability to change the values in the lists. Listing 15-24 shows that by using a 
RefCell<T> in the Cons definition, we can modify the value stored in all the lists:
*/
// Import necessary traits and types from the standard library.
use std::cell::RefCell;
use std::rc::Rc;

// Define an enumeration named "List" to represent a linked list.
#[derive(Debug)]
enum List {
    // Variant "Cons" contains an Rc (Reference Counted) reference to a RefCell containing an i32 and a reference to another "List."
    Cons(Rc<RefCell<i32>>, Rc<List>),
    // Variant "Nil" represents the end of the list.
    Nil,
}

// using the Cons and Nil from List Enum
use crate::List::{Cons,Nil};

fn main() {
    // Create an Rc (Reference Counted) reference to a RefCell containing the integer 5.
    let value = Rc::new(RefCell::new(5));

    // Create a variable "a" with an Rc reference to a "Cons" variant.
    // It shares ownership of the "value" Rc and points to the end of the list represented by "Nil."
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // Create a variable "b" with a "Cons" variant that contains a new Rc reference to a RefCell containing the integer 3,
    // and an Rc reference to the same list as "a," effectively sharing the same tail of the list.
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    // Create a variable "c" with a "Cons" variant that contains a new Rc reference to a RefCell containing the integer 4,
    // and an Rc reference to the same list as "a," sharing the same tail as "b."
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    // Mutate the value inside the RefCell referred to by "value" by adding 10 to it.
    // this change will be applied to all a, b, c as they are referencing to it
    *value.borrow_mut() += 10;

    // Print the values of the lists "a," "b," and "c" after the mutation.
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Note: To see the example of RefCell<T>, it's available in the "lib.rs"