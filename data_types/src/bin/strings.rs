#[allow(unused)]
// Two types of strings exists in Rust
// 1. String Slice - Represented by &str 
//      -> Stored in Stack i.e. Program Binary / Static Memory
//      -> Somtimes a heap reference, sometimes embedded in code
//      -> Immutable(Exceptions)
// 2. String - Represented by String 
//      -> Stored in Heap 
//      -> Mutable
fn main() {
    // creating string slices and strings
    let example_str = "Rusty"; // this is a string slice also called string literal
    let example_string = String::from("Hello"); // this creates a string with value Hello
    
    // converting &str to String
    let str_to_string_var = String::from(example_str); // "from" fn converts &str to String
    let str_to_string = example_str.to_string();       // "to_string"converts &str to String
    let str_to_string_2 = "Rustling".to_string();      // works the same
    // here the "Rustling" by default is considered as a &str type, 
    // hence we use .to_string() to convert to String

    // converting String to &str and &String
    // here &String is just normal reference to the String, it's just a bare pointer
    // bare pointer -> stores just the address of the data
    // &str is reference to the slice, its a fat pointer
    // fat pointer -> stores the address of the data + the str length
    // link: https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
    let string_to_str = &example_string; // here string_to_str is a reference to example_string
    let string_to_str_2 = example_string.as_str(); // string_to_str is ref to the string slice

    // combining strings literals 
    let combining_str_literals = ["Hi ",example_str].concat();     // returns String type 
    let combining_str_with_format = format!("{} {}","Good","Bye"); // returns a String not &str
    println!("Combining String Literals: {}",combining_str_literals);

    // to add a &str and String, the order should be String followed by &str
    // new_var = String Var + &str Var -> works
    // new_var = &str Var + String Var -> panic
    let add_str_and_string = example_string + example_str; 
    println!("Combining String and &str {}",add_str_and_string);

    // creating new strings
    let mut mut_string = String::new(); // creates a new empty mutable string
    mut_string = "New".to_string(); // assinging value to the new string 

    // Pushing strings
    mut_string.push_str("three");     // appends the str slice "three" to String "two"
    mut_string.push_str(example_str); // same
    mut_string.push('a');                 // push() is used to append a char type

    println!("Pushing str,char to String: {}",mut_string);

    // Adding Strings Together
    // when adding strings, from second string, you need to use &
    // i.e convert the String to String Literal
    let a = String::from("one");
    let b = String::from("two");
    let combined = a + &b + &mut_string + "hello";
    println!("Adding Multiple Strings: {}",combined);
    
    // creating substrings
    let example_str_2 = "Hello";
    let example_string_2 = "World".to_string();

    let substring_from_str = &example_str_2[..2]; // He
    let substring_from_string = &example_string_2[2..]; // rld

    // getting char from str / String - process is same
    let char_by_index = example_str_2.chars().nth(1); // e
    // &example_str_2.chars().nth(1) returns an Enum

    match char_by_index{
        Some(c) => println!("Char By Index: {}",c), // if the nth char exists
        None => {} // Do nothing
    }

    // other way
    if let Some(chh) = example_str_2.chars().nth(1){
        println!("Found a char {}",chh);
    }
    
    

}