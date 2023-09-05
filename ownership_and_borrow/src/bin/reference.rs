// --- References - Non Owing Pointers ---
// References are pointers that do not own anything as they dont own the data they point to
// References are used when you want to use a var(stored on heap) multiple times

// here the var name does not take the ownership of the arguement passed to it
// instead it references the data(hence & before String)
// "name" is a reference pointing to my_name and my_name is String pointing to a data in heap
fn greet(name: &String){
    println!("Greetings, {}",name);
}
#[allow(unused_variables)]
#[allow(unused_mut)]
fn main(){
    let my_name = String::from("Lucifer");

    // here we passing the reference to("borrow") the String using & i.e. &String
    // when passing a reference a copy of "Lucifer" is being sent to the greet() function
    // hence we can call the function multiple times with reference
    greet(&my_name);
    // greet(&my_name); -> works as we are sending a copy of the var but not actual var
    // greet(&my_name); -> works as we are sending a copy of the var but not actual var
    // println!("{:?}",my_name); -> this works because my_name still has the ownership

    // R - Read, O - Own/Owner, W - Write

    // Immuate Borowing/Referencing - Shared Reference
    // lets create a mutable vec
    let mut v = vec![1,2,3,4]; // v -> RWO

    // let's borrow/reference an element from the vector
    let ele_two = &v[1]; // v -> R, ele_two -> RO, *ele_two -> R
    // here ele_two is an immutable reference/borrow, i.e. doesn't have W permission
    // an immutable reference allows aliasing but not mutation
    // this implies that we can access vector elments from both "v" and "ele_two"
    // but we cannot change the vec elements with ele_two

    // *ele_two+=1; -> doesn't work as *ele_two has only R permission

    // now if we modify "v", we cannot use ele_two, as a new addr will be allocated in heap for v
    /* 
        let's perform a mutation - i.e. alter "v"
        v.push(25); // this is a mutable borrow, here .push does a mutable borrow of v
        println!("{}",ele_two) -> throws an error
    */
    // so if we want to use "ele_two", it must be done before making any changes to the "v"

    // Impotant: Immutable Borrow/References allows aliases but doesn't allow mutation


    // ---- Mutable Borowing/Referencing - Unique Reference ----
    // Important: Mutable Borrow/References allows for mutation but doesn't allow aliases
    let mut v = vec![1,2,3,4]; // v -> RWO
    println!("----- Mutable Borrowing Concept -------");
    println!("Initial Vector {:?}",v);

    // let's do a mutable borrow/reference an element from the vector
    let ele_two = &mut v[1]; // v -> (), ele_two -> RO, *ele_two -> RW
    // here we see that the vec "v" doesn't even have a R permission,
    // this implies that we cannot acces vec via "v"(temporarily) unless ele_two is destroyed
    // hence "v" is no more an alias temporarily
    // *ele_two has W permision implies we can modify the element in the vec

    /* 
        let's try to access vec via v
        println!("{:?}",v); -> this fails because the ele_two still exists below it
        println!("{}",*ele_two); -> if using only this fn, it works as it has R permission
    */

    // let's mutate v[1] in vec
    // v.push(34); it fails as we have already done a mutable borrow in line54
    *ele_two+=1; // it works as *ele_two has W persion, thus mutates v[1]
    println!("Updated Second Element: {}",*ele_two); 
    println!("Updated Vector{:?}",v); 
    // the above works because we no longer use ele_two below this statement,
    // i.e. the lifetime of ele_two ended with line 68 
    // hence v gets back RWO permission
    v.push(24); // same explination as above
    println!("Updated Vector{:?}",v);    

}