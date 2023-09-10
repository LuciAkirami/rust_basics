#![allow(unused_variables,dead_code)]
/*
-------- TRAITS ------
A type’s behavior consists of the methods we can call on that type. Different types share the s
ame behavior if we can call the same methods on all of those types. Trait definitions are a way to 
group method signatures together to define a set of behaviors necessary to accomplish some purpose.
*/

struct Tiger{
    num_legs: u8
}

struct Monkey{
    num_legs: u8
}

struct Eagle{
    num_wings: u8
}

// here we create a trait MammalAnatomy having the following functions
// instead just method signature, we can also define a fn with default behaviour
pub trait MammalAnatomy{
    fn how_many_legs(&self);
}

// here we create a trait BirdAnatomy having the following functions
pub trait BirdAnatomy{
    fn how_many_wings(&self);
}


// here we create a trait LivingThing having the following functions
// we also declared a default behaviour
// pub imples we can use this trait outside of this file
pub trait LivingThing{
    fn is_this_living(&self){
        println!("This is a living thing")
    }
}
// here we are implementing MammalAnatomy trait on the type Tiger
impl MammalAnatomy for Tiger{
    fn how_many_legs(&self){
        println!("Number of Legs: {}",self.num_legs)
    }
}

// here we are implementing MammalAnatomy trait on the type Monkey
impl MammalAnatomy for Monkey{
    fn how_many_legs(&self){
        println!("Number of Legs: {}",self.num_legs)
    }
}

// here we are implementing BirdAnatomy trait on the type Eagle
impl BirdAnatomy for Eagle{
    fn how_many_wings(&self){
        println!("Number of Wings: {}",self.num_wings)
    }
}

impl LivingThing for Tiger {}
impl LivingThing for Monkey {}
impl LivingThing for Eagle {}


// ---------- Trait Bounds -----------
// Trait Bounds is restricting a generic type based on the trait implementation
// Implementing a generic func that only takes the Types that implement a specific trait
// the fn tells that, it is generic over any type "T", that implements the "Trait" "MammalAnatomy"
fn is_this_mammal<T: MammalAnatomy> (mammal: &T){
    println!("Yes this is a mammal");
}
/*
fn is_this_mammal<T: MammalAnatomy> (mammal: &T){}

this can be re-written as 

fn is_this_mammal(mammal: &impl MammalAnatomy){} -> more easy to write, a syntactical sugar

we can also specify more than one trait bound
fn is_this_living<T: MammalAnatomy + BirdAnatomy> (living: T)
this tells the function takes in any type T that implements both MammalAnatomy and(not or) BirdAnatomy


fn some_func() -> impl trait_name{}
this fn returns a generic type that implements the trait "trait_name"
Note: this fn only works when all the type that are returned in this func have the same function definition
for that trait
Note: If i return any of two types "lets say a and b" that implement the trait "trait_name". If the type "a",
implements the fns in "trait_name" differently than type "b", then rust will throw an error
*/

/*
------ WHERE Caluse -----------
when defining trait bounds for eacch type, they can get big like the below
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

so we can use a "Where" caluse here
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
    
*/

// this function works on any generic type that implements BirdAnatomy trait
fn can_fly<T> (bird: &T)
where
    T: BirdAnatomy
{
    println!("Birds can fly")
}

fn main(){
    let tiger = Tiger{num_legs: 4};
    tiger.how_many_legs();
    tiger.is_this_living();
    
    is_this_mammal(&tiger); // it works as tiger implements Trait MammalAnatomy

    let monkey = Monkey{num_legs: 2};
    monkey.how_many_legs();
    monkey.is_this_living();
    is_this_mammal(&monkey); // it works as monkey implements Trait MammalAnatomy

    let eagle = Eagle{num_wings: 2};
    eagle.is_this_living();
    eagle.how_many_wings();
    can_fly(&eagle);
    // is_this_mammal(eagle); panics, as eagle doesn't implement the Trait MammalAnatomy

}