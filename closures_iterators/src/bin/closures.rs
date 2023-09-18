#![allow(unused_mut)]
fn main(){
    // "|| give_num()" is a closure. The unwrap_or_else function, unwraps a Some(T) if
    // a Some(T) is given or computes a value from the closure i.e "|| give_num()"
    // Here return_option returns None, hence value from the closure is calculated
    // "||" imples no value and the closure calls give_num() which returns 25 and then assigned to x
    let x = return_option(15).unwrap_or_else(|| give_num());
    println!("{x}");

    // Here return_option returns Some(97), hence the closure is not called
    let x = return_option(97).unwrap_or_else(|| give_num());
    println!("{x}");

    // Defining closures to be used later
    // Here when defininf clousre we need to mention the input type as well as the output type
    let incr_by_5 = |num: i32| -> i32 {num+5};

    // Calling the above closure
    let x = 10;
    let y = incr_by_5(x);

    println!("{y}");

    /*
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    this gives error because in line 26, the closure takes String as input and the compiler
    will set the input type of the closure to String, but in the below line we pass a int
    which isn't a string, hence rust will panic

    let f = |_| (); // sometimes called the "toilet closure"
    let s = String::from("Hello");
    f(s);

    here when we call f(s), "s" will be dropped and no longer valid
    */

    // ------ Immutable Borrow by Clousure --------
    println!("\nImmutable Borrow by Closure\n");
    let word = String::from("Henlo");
    println!("Before defining closure: {:?}", word);

    let only_borrows = || println!("From closure: {:?}", word);

    println!("Before calling closure: {:?}", word);
    only_borrows();
    println!("After calling closure: {:?}\n", word);

    // ------ Mutable Borrow by Clousure --------
    println!("Mutable Borrow by Closure\n");
    let mut word = String::from("Henlo");
    println!("Before defining closure: {:?}", word);

    let mut mutable_borrows = || word.push_str("World");

    // println!("Before calling closure: {:?}", word); panics because "word" is already borrowed mutably
    mutable_borrows();
    println!("After calling closure: {:?}\n", word);

    // ------ Closure Taking Ownership --------
    println!("Moving Ownership inside Closure\n");
    let mut word = String::from("Henlo");
    println!("Before defining closure: {:?}", word);

    let mut mutable_borrows = move || println!("From closure: {:?}\n", word);
    
    mutable_borrows();
    // println!("After calling closure: {:?}", word); panics because "word" is moved in line 64

    // ------ Clousure Capturing Variables from it's Environment/Scope --------
    
    let x = 20;
    let add_x = |num: i32| num + x; // here the closure is able to access "x"
    println!("Added by x: {}\n",add_x(19));

    // ------ Passing Closure to a Function ---------
    let add = |x,y| x + y;
    calculater(5, 10, add);

    // the below also works
    // fn add_two(x: i32, y: i32) -> i32 {x+y};
    // calculater(5, 10, add_two);

    // Closure capturing env variable 
    // let z = 35;
    // let add = |x,y| x + y + z;
    // calculater(5, 10, add); panics as "add fn" captures a variable "z" in its definition



}


fn return_option(i: i32) -> Option<i32>{
    if i > 20{
        return Some(i)
    }
    None
}

fn give_num() -> i32{
    25
}

// fn(i32,i32) -> i32 is a fn pointer that can take both functions and closures 
// But the condition for closure is that, it must not use capture any environment variables
fn calculater(x: i32, y: i32, add_func: fn(i32,i32)->i32){
    let sum = add_func(x,y);
    println!("Closure Passed to a Function");
    println!("Sum of {} and {} is {}",x,y,sum);
}