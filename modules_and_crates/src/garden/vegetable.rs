// this is a sub module
// a function inside a sub-module
pub fn tomato(){
    println!("I'm a Tomato\n")
}

// a struct inside a sub-module
// for each data in struct we need to mention if its public or not
pub struct Banana {pub ripen: bool} // pub ripen implies, we can edit ripen in main.rs

impl Banana{
    pub fn is_banana_ripen(self: &Self){ // it imples slef is referencinf to Banana
        if self.ripen{
            println!("Banana is Riped\n");
        }
    }
}

// if the enum is public, all of its variants are public
pub enum Leafy{
    Spinach,
    Cabbage,
    Coriander
}

