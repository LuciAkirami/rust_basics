// Crate Roots
// src/main.rs -> crate root for binary crate
// src/lib.rs -> crate root for library crate

// There are two types of crates, Binary Crate 
// Binary Crates compiled to executables like command line programs and servers
// these contain main() fn. main.rs is a binary crates
// the .rs files in the src/bin, each file is a separate binary crate

// Library Crate
// Library Crates are not complied to executables and they don't contain a main() fn
// they define a functionality to be used by other programs, eg "rand" is a lib crate
// to create a library file we can use cargo new lib_name --lib
// usually binary crates depend on library crates for fns

// Package -> A bundle of one or more crates that provides a set of functionality
// package contains toml file that tells how to build that crate
// package contains one or more binary crates but only a single library crate
// the name of the package is the name of file that you used to create the package
// eg: cargo new my_package -> here my_package is the package name

#![allow(dead_code)]

// Using a module
pub mod garden; // here adding pub tells the compiler to include code it finds in this module
// to use the code within the module, the compiler will look for
// a file src/graden.rs or src/garden/mod.rs which contains the module's code
// these are two way to create modules, another way is to create the module in main.rs

// the below is called in-line module, where module and it's code are defined
// in the file where it's used, useful when grouping similar functions
mod module_in_main{
    fn this_module_fn_is_in_main() {} // this is private function as no "pub"
}

// Example of inline module - Animal module containing animal function
// pub -> imples the fn is public and can be called by any file
mod animals{
    pub fn say_meow() {println!("Meow")}
    pub fn say_woof() {println!("Woof")}
}

// Using the "use" statement to reduce the lines of code
use garden::vegetable;
// now to call the functions/code within the vegetable module, we need not use
// garden::vegetable::fn_name, instead we can use vegetable::fn_name

// garden::vegetable::fn_name, with this we can directly use fn_name without vegetable

// importing a struct from a sub-module
use garden::vegetable::Banana;

// importing war module
mod war;

// ------ Nested Modules ----------
mod parent{ // no need to use pub as we have created this module in file where we use it
    fn parent_fn(){
        println!("This is parent function\n")
    }

    pub mod child{ // requires "pub" as its inside a module
        pub fn child_fn(){
            // calling parent fn in child fn using "super"
            println!("\nInside child");
            super::parent_fn();
        }           
    }
}

fn main() {
    // using in-line module functions
    animals::say_meow();
    // module_in_main::this_module_fn_is_in_main(); error as its private fn

    // running a code from module
    garden::garden_name();

    // running a code from sub module
    vegetable::tomato();

    // using a struct defined in a sub-module
    let my_banana = Banana{ripen: true};
    vegetable::Banana::is_banana_ripen(&my_banana);
    //println!("Is my Banana Ripen? \nAns: {}\n",my_banana.ripen);

    // using functions from war/mod.rs
    war::launc_attack();

    // calling parent code from a child module
    parent::child::child_fn();


}


// Absolute path
// An absolute path is the full path starting from a crate root; for code from an 
// external crate, the absolute path begins with the crate name, and for code from 
// the current crate, it starts with the literal crate.
// crate::animals::say_meow();
// the above is an abosolute path, where the crate gets resolved to our project name

// Relative path
// animals::say_meow();


// ----- USE Statement ------
/*
-> example -1
use std::io;
use std::io::Write;

this can be re-written as
use std::io::(self,Write) -> self corresponds to std::io

-> example - 2
use std::cmp::Ordering;
use std::io;

this can be re-written as
use std::(cmp::Ordering,io);
*/

// ------- Mod File Paths --------------

/*
-> A module can be created in 2 ways
1. src/mod_name.rs
2. src/mod_name/mod.rs (old style)

-> A sub module can be created in 2 ways
1. 
src/mod_name.rs
src/mod_name/sub_module_name.rs

2. 
src/mod_name/mod.rs
src/mod_name/sub_module_name/mod.rs (old style)
*/