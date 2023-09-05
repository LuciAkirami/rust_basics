/*
// -------- STRUCTS -------------
Structs in Rust are similar to objects in other languages but have few differences
-> Rust doesn't have any inheritance
-> Rust does has methods
-> Rust has "Traits" - Similar to polymorphism in OOPs
-> Follow PascalCase for naming Structs
eg: snake_case -> Snake Case, all small letters, words separated by _
    camelCase -> first letter of first word small, for remaining Capital, no space between words
    PascalCase -> first letter of every word is Capital and no space between words
*/

// mod will let us use the struct in car_struct.rs
mod car_struct;          // mod is short for module, here car_struct.rs is a module
use car_struct::CarInfo; // will let us use the struct CarInfo from car_struct.rs

#[allow(dead_code)] // supress unused structs, only supresses in ComputerInfo Struct
#[allow(unused_variables)] // this supresses warnings only in ComputerInfo struct
struct ComputerInfo{   // best practice is to order fields in alphabetical order  
    cost: f32,
    id: i32, 
    is_64bit: bool,
}

#[allow(dead_code)] // this supresses warnings only in User struct
struct User{
    name: String,
    age: u8,
    job: String,
}

#[allow(unused_variables)] // as its placed above main fn, it supress warnings in main fn
fn main(){
    let my_computer = ComputerInfo{
        cost: 500.0,
        id: 25,
        is_64bit: true, 
    };

    // my_computer.id = 78; -> fails as its not mutable
    // lets create mutable structs

    let mut my_computer = ComputerInfo{
        cost: 500.0,
        id: 25,
        is_64bit: true,
    };

    // accesing struct fields and changing it
    my_computer.id = 14;

    println!("My Computer Cost: {}",my_computer.cost);

    // using data from one struct element to another
    // lets assume we have my_computer_2 where the only difference is ID
    // so we can just pass value for ID and copy data from my_computer for
    // remaining values.

    let my_computer_2 = ComputerInfo{
        id: 100,
        ..my_computer 
    };
    // use ..struct_name to move remaining struct values from my_computer 
    // to "my_computer_2". 

    // ------- Ownership in Structs -------------
    let user_1 = User{
        name: "Lucifer".to_string(),
        age: 50,
        job: "Dev".to_string(),
    };

    let user_2 = User{
        name: "Luffy".to_string(),
        job: "Manager".to_string(),
        ..user_1 // user_2.age = user_1.age -> Copy operation(int stored in stack)
    };
    // here we are using the age from "user_1" to "user_2", as "age" is a primitive datatype
    // a copy of age is done from "user_1" to "user_2", hence "user_1" still exists
    // println!("{}"user_1.age); -> works
    let user_3 = User{
        name: "Luffy".to_string(),
        age: 45,
        ..user_1 // user_3.job = user_1.job -> "move" operation -> change in ownership
    };
    // here we are using "job" from "user_1" to "user_3". "job" is compound data type
    // so it exists on heap, hence here instead of "copy", a "move" operation is performed btw
    // "user_1's job" to "user_3's job". Hence ownership of job is changed and hence
    // user_1.job becomes invalid(deallocated) but user_1.name, user_1.age will still work
    // println!("{:?}",user_1.job);

    //-----------------------------------------------------------------------------
    println!("--------------------------------------------------------------------");
    // --- Using Struct From Other Files ---
    let my_car = CarInfo{
        country_code: 'U',
        model_num: 54,
        mileage: 10,
    };

    // using CarInfo new() fn to create new variable
    let mut new_car = CarInfo::new('I', 90);
    println!("Car with Country Code {}, Model Num {}",new_car.country_code,new_car.model_num);

    // using methods of structs
    println!("Car mileage: {}",new_car.mileage);
    new_car.increase_mileage();
    println!("Car Mileage Incrby 5: {}",new_car.mileage);

    // printing the struct with debug
    println!("Debug Print the Struct {:?}",new_car);

}