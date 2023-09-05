// the "pub" before the struct makes the struct public, thus allowing any file to use it
// the "pub" before the fields make them public, thus allowing any file to store value
// in these fields
#[derive(Debug)] // this is required to print the struct directly
pub struct CarInfo{
    pub country_code: char, // if there is no pub -> field is private and cannot be modified
    pub model_num: u8,
    pub mileage: u8,
}

// implementing fn for CarInfo
#[allow(dead_code)]
impl CarInfo{
    // pub because we want new() fn to be used anywhere
    // here new() is an associated fn as it doesnt take self/&self/Self as input
    // here Self refers to type CarInfo, we can write CarInfo inplace of Self too
    pub fn new(country_code: char, model_num: u8) -> Self{ 
        Self { 
            country_code, 
            model_num,
            mileage: 10
        }
        // "country_code: country_code" or just "country_code"
        // both are correct as the "parameters" and "field" names are same
    }

    // this fn takes the struct as mutable and increases the mileage field of struct
    pub fn increase_mileage(&mut self) {
        self.mileage += 5
    }
}