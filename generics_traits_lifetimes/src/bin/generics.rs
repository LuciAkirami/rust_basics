#![allow(unused_variables,dead_code)]
// Generics is the way of making the code generic to different data types
// We use generics with functions and structs, which we can then use with
// different concrete data types

/*
Example:
fn largest<T>(list: &[T]) -> &T {
We read this definition as: the function largest is generic over some type T. 
This function has one parameter named list, which is a slice of values of type T. 
The largest function will return a reference to a value of the same type T. 

----- Generics for Structs --------
struct Point<T>{
    x: T;
    y: T;
}
Here we say the struct Point is generic over a type T
if:
let point = Point{x: 30, y: 60}, T becomes i32
let point = Point{x: "thirty", y: "sixty"}, T becomes &str

----- Using Multiple Generics --------
struct student<T, U>{
    name: T;
    age: U;
}
struct Student is generic over type T and U
eg: let student_1 = student{name: "Lucifer", age:32}, here T = &str and U = i32

----- Generics are actually used in Options and Results Enums -----
enum Option<T> {
    Some(T),
    None,
}

Option is generic over type T, implies it might give something of type T or nothing

enum Result<T, E> {
    Ok(T),
    Err(E),
}
The Result enum is generic over two types, T and E, and has two variants: Ok, 
which holds a value of type T, and Err, which holds a value of type E
*/

// ----- Generics in Method Implementations -------
struct Point<T> {
    x: T,
    y: T,
}

// By declaring T as a generic type after impl, Rust can identify that the 
// type in the angle brackets in Point is a generic type rather than a concrete type
// Methods written within an impl that declares the generic type will be defined 
// on any instance of the type, no matter what concrete type ends up 
// substituting for the generic type.

impl<T> Point<T> {
    fn x(&self) -> &T { // this function returns a reference to a generic type
        &self.x
    }
}

// Here the below call_me() method only works on the Point struct having the
// type as u8
impl Point<u8>{
    fn extract_x(&self){
        println!("X is {}",self.x);
    }
}

struct NewPoint<X1, Y1>{
    x: X1,
    y: Y1
}
// this imples the methods are generic over any type X1 and X2
impl<X1, Y1> NewPoint<X1, Y1> {
    // it imples the fn is generic over a Point of any type X2, Y2
    // and then it return a mix of types from Point<X1, Y1> and Point <X2, Y2>
    fn mixup<X2, Y2>(self, other: NewPoint<X2, Y2>) -> NewPoint<X1, Y2> {
        NewPoint {
            x: self.x,
            y: other.y,
        }
    }
}
fn main(){
    let point_1 = Point{x: 20.0, y:50.0};
    let x = point_1.x;
    // point_1.extract_x(); fails as the method is only defined for type u8
    
    let point_2 = Point{x: 25 as u8, y:70 as u8};
    let x = point_2.x;
    point_2.extract_x();

    let point_3 = NewPoint{x: 20, y: 90};
    let point_4 = NewPoint{x:4.3, y: 9.8};
    let point_5 = point_3.mixup(point_4);
    // point_5 contains the types a mix of point_4 and point_5
}