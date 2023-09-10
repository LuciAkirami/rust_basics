// --------- Strings --------------
#![allow(unused_variables)]
fn main(){
    let mut a = String::new();
    a.push_str("hello"); // "a" contains hello

    let mut x = "Hello ".to_string();
    let y = "Word";
    x.push_str(y); // here "x" = Hello World
    // println("{}",y); "y" is still valid because in line 8, a reference of y is used

    let s1 = "Hello ".to_string();
    let s2 = "World".to_string();
    let s3 = s1 + &s2; // s1 is moved and no longer valid
    // println!("{}",s1); fails as s1 no longer owns "Hello " as its "moved" to s3

    /*
    what happens thwn s1 + &s2 is done, an add function is called
    which looks like the below

    fn add(self, s: &str) -> String {
    
    here we see it takes it arguments, self is s1, and there is no reference,
    so the first element is moved and ownership is lost

    For the second arguement we see &str, hence a reference is used and the second 
    element will not lose ownership and hence s2 is list valid

    We also see that we cannot add two Strings directly, we compulsary have to
    make the second and any element after the first element as a reference
    Here when we do &s2 -> &String -> (coercion when add fn called) -> &str
     */ 

    // another way to add String -> Without loss of ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // s = "tic-tac-toe"
    // here format takes references of all three strings and returns a new string
    // hence s1, s2 and s3 are all valid after the line 38

    // Strings cannot be indexed
    // s1[0] or &s1[0] will result in a panic, but there are other methods
    
    // iterating through characters in string
    for char in s1.chars(){
        println!("{}",char);
    }
    
    // itering through String and returns raw bytes
    for byt in s1.bytes(){
        println!("{}",byt); // returns ASCII value of t,i,c
    }

}