// garden.rs is a module
// vegetable is a sub-module, that is a module within a module

pub mod vegetable;
// the "pub" tells the compiler to include the code from the vegetable module
// to use the code within vegetable module, the compiler looks
// for a file garden/vegetable.rs

pub fn garden_name(){
    println!("This is Victoria Gardens\n");
}