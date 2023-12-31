// ------------- Option Enum ---------------
/* 
-> Option enum encodes very common scenario where a value could be something or nothing

enum Option<T> { 
    None,
    Some(T),
}

-> So option has two variants "None" which is nothing and "Some(T)" which returns something.
-> Option::Some takes in any data type, hence its generic over any Type T, hence <T>
-> Option is available in prelude, so we don't need to import it, we can Some and None directly
*/

#![allow(unused_variables)] // #! implies it applies to whole file
fn main(){

    let x = Some(45);
    let y = Some("hello");
    // let z = None; this fails, the compiler tells use to provide a type for Option
    // Option<T> needs to have a type, so even if we give None, we need to explicity specify a type
    let z: Option<i32> = None; // here we have explicity given a type i32 to Option
    
    // Adding an Option<i32> and i32
    let a = 5;
    // let b = a + x; fails, we cannot add them, because x is Option<i32> and 
    // as we know an Option<i32> can either be something "Some(i32)" or Nothing None
    // so as it can be Something or Nothing, we get an error as we cannot add if its Nothing
    // one way to perform this is to extract the 45 from Some(45)

    let b = a + x.unwrap(); // the unwrap() extracts T from Some(T)
    // only use the above is your sure the Option variant is Some(T)
    // let c = a + z.unwrap(); this doesn't show error while writing code
    // but when we run it, it panics as we cannot unwrap a "None"
 
    // ------ Option Implementations ---------
    let x = Some(2);
    let y: Option<u8> = None;

    // is_some(), returns true if Option<T> is Some<T> else false
    assert_eq!(x.is_some(),true); 

    // is_none(), returns true if it's None variant of Option<T> else false 
    assert_eq!(y.is_none(),true); 

    // as_mut(), converts &mut Option<T> to Option<&mut T>
    // => Some(&mut T), this let's us edit value in Some
    let mut x = Some(10);
    // if x contains Some, then edit the value in x
    match x.as_mut(){
        // here k binds to the value in Some, hence k = 10
        Some(k) => *k = *k * 2,
        None => (),
    }
    assert_eq!(x,Some(20)); // no error occurs as its true

    // Option in Match - Primitive Data Type
    let x = Some(5);
    match x{
        Some(i) => println!("X contains {}",i),
        None => (),
    };
    println!("{:?}\n",x);
    // x is valid, because x contains primitive data and match "x" will copy x instead move

    // Option in Match - Non Primtive
    let x = Some("Hello".to_string()); // "x" contains non primitive data
    match x{ // match x, this will move "x" to match
        Some(i) => println!("X contains {}",i), 
        None => (),
    };
    // println!("{:?}",x); this fails, because we already "moved" "x" to "match"
    
    // Solving the Above using Clone
    let x = Some("Hello".to_string());
    match x.clone(){ // match x, this will move "x" to match
        Some(i) => println!("X contains {}",i), 
        None => (),
    };
    println!("{:?}\n",x); // this works because are are "Cloning" the "x" instead "move"

    // Referencing Option - Better Approach
    let x = Some("Hello".to_string());
    match &x{ // match x, this will move "x" to match
        Some(i) => println!("X contains {}",i), // Notice here "i" is &String
        None => (),
    };
    // Rust will “push down” the reference from the outer enum, &Option<String>, 
    // to the inner field, &String
    println!("{:?}\n",x);

    // if-let, an alternative to "match"
    // if we want to match only to a single patter we can use if-let
    let x = Some(5);

    if let Some(var) = x{
        println!("Some contains {}",var);
    }

    // here we see that we do not need to worry about the None case
    // in match we have to do None => () or _ => (), so with if let, we don't need to write it
    // if-let is kinda synatic sugar of match, only used when we want to match a single pattern

    


}