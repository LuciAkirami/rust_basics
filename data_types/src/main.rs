#![allow(unused)]
// the file contains all the data types included in rust
fn main() {
    // ----- basic data types -------
    let int_type = 5;              // i32 - 32bit integer by default
    let float_type = 2.0;          // f64 - 64bit float by default
    let character_type = 'Z';     // character
    let boolean_type = true;      // bool data type
    let float_32_type: f32 = 4.5;       // f32 float data type
    let float_32_type = 10_f32;    // other way to assign float
    let float_64_type = 10f64;     // other way to assign float
    let float_32_type = int_type as f32;       // other way to assign float / typecast int to float
    println!("Max value of u16: {}",std::u16::MAX); // printing max value that a u16 integer can store
    println!("Min value of i32: {}",std::i32::MIN); // printing min value that a i32 integer can store
    
    // ---- Compound Data Types ----
    // Tuples - Can contain data of different data types
    let my_tuple: (u8, char, f32) = (25, 'a', 2.98);
    let (unsign_int_type, char_type, float_type) = my_tuple; // destructing tuple to individual elemnts
    let twenty_five = my_tuple.0;                 // accessing individual ele
    println!("Accesing ele in tuple: {}",my_tuple.1);

    // Arrays - Can contain data of single data type
    let num_array = [1,2,3,4,5];
    let same_array = [2;5]; // same_array = [2,2,2,2,2]
    let str_array = ["one","two","three"];
    println!("Accessing ele in array: {}",num_array[2]);

    // Tuple of Arrays
    let new_tuple = ([3;5], [9;3]);             // second_array = [9,9,9]
    let (first_array,second_array) = new_tuple;   // first_array = [3,3,3,3,3]
    let array_element_inside_tuple = new_tuple.1[2];             // Accesing third ele of second array
    println!("Accesing Third Ele of Second Array in Tuple: {}",new_tuple.1[2]);

}
