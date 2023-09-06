// the "pub" before the struct makes the struct public, thus allowing any file to use it
// the "pub" before the fields make them public, thus allowing any file to store value
// in these fields
// with "derive" attribute we can use pre-defined traits
#[derive(Debug)] // this is required to print the struct, i.e. use the output formatter "Debug"
pub struct CarInfo{
    pub country_code: char, // if there is no pub -> field is private and cannot be modified
    pub model_num: u8,
    pub mileage: u8,
}

// implementing fn for CarInfo. "impl" is used to implement fn for a struct
// all fn defined in impl are calles associated fn as they are associated 
// to the type named after impl and we can have multiple "impl blocks" for "same type"
#[allow(dead_code)]
impl CarInfo{
    // pub because we want new() fn to be used anywhere
    // here new() is an associated fn and is a constructor as it returns a new 
    // instance of the struct
    // here Self is an alias to the type that the "impl" block is for, here its CarInfo, 
    // we can write CarInfo inplace of Self too
    pub fn new(country_code: char, model_num: u8) -> Self{ 
        Self { 
            country_code, 
            model_num,
            mileage: 10
        }
        // "country_code: country_code" or just "country_code"
        // both are correct as the "parameters" and "field" names are same
    }

    // here &self is short for "self: &Self", => we are borrowing the instance that call it
    pub fn get_car_info(&self){
        println!("Car Information");
        println!("Car model number is {}",self.model_num);
        println!("Car country code is {}",self.country_code);
        println!("Car mileage is {}",self.mileage);
    }

    // this fn takes the struct as mutable and increases the mileage field of struct
    // here basically we doing a mutable borrowing of the "instance" that calls it
    pub fn increase_mileage(&mut self) {
        self.mileage += 5
    }

}