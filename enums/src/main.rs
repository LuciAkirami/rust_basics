#[allow(dead_code)]
// an Enum contains a list of items
enum Payment{
    Cash,
    CreditCard,
    DebitCard,
}

// Enums that store a value within each item
#[allow(dead_code)]
enum DataEnums{
    Integer(u8),
    Float(f32),
    Character(char),
    Stringy(String),
    Person{name: String, age: u8},  // Person item stores an enum, 
    Structure(Home),                // Structure item stores a Struct
}

struct Home{
    area: String,
    cost: f32,
}


fn main() {
    let payment_method = Payment::CreditCard;

    match payment_method{
        Payment::Cash => println!("Paying with Cash"),
        Payment::CreditCard => println!("Paying with Credit Card"),
        Payment::DebitCard => println!("Paying with Debit Card"),
    }
    // Note: the match statement should contain all the items of an Enum
    // Else it will throw an error, one way to avoid is use _

    match payment_method{
        Payment::Cash => println!("Paying with Cash"),
        _ => (), // here _ -> every other case & "()" -> Do nothing
    }

    // using enums that store values
    let person_item = DataEnums::Person { name: "Lucifer".to_string(), age: 20};
    let structure_item = DataEnums::Structure(Home { area: "Swiz".to_string(), cost: 5000.5 });
    let float_item = DataEnums::Float(34.5);

    process_datatype(person_item);
    process_datatype(structure_item);
    process_datatype(float_item);

    // using enums that store values with match

}

fn process_datatype(datatype: DataEnums){
    match datatype{
        DataEnums::Person { name, age } => {
            println!("Person with name {}, age {}",name, age);
        }
        // the var name inside items can be anything, here i took it as "house"
        DataEnums::Structure(house) => {
            println!("Home with place {}, cost {}",house.area, house.cost);
        }
        DataEnums::Float(my_float) => {
            println!("Float with value {}",my_float);
        }
        _ => ()
    }
}