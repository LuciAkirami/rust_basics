#![allow(unused_variables)]
use std::collections::HashMap;
// HashMap Rule
// All keys should be of same type and all values of same type

fn main(){
    let mut student = HashMap::new();
    student.insert("Branch".to_string(), "CSE".to_string());
    // student.insert("Age".to_string(),22);fails as 22 is int not String
    student.insert("Age".to_string(), "22".to_string());

    // Accesing Values of Hashmap
    let age_key = "Age".to_string();
    let age_default = "22".to_string();
    let age = student.get(&age_key).clone().unwrap_or(&age_default);
    // so in line 13, to get the value of the key "age", use the get() method
    // on student and then pass the key as a referene to it.
    // as the "value" is a string, we can't copy it, hence we use the clone() to
    // clone the "value" and then use the unwrap_or() method. After we call get()
    // type returned is an Option<&T>. The clone() converts Option<&T> to Option<T>
    // Hence to get the Actual String, we use the
    // unwrap. And if there is no "key" named "age", then we will set the value of
    // the "variable age" to "22", for this we use the "unwrap_or", so it unwraps
    // if its Some<T> or if get returns None it will pass the default val to that var
    // and as it's a string, we pass the reference to the String to "unwrap_or"
    println!("{:?}",age);

    // Another example
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 30);
    // value of "Yellow" changes from 50 to 30

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // this one is similar to above but here the "values" are of type integer
    // hence we will use the "copied()" fn instead of clone as integers
    // can be copied and in unwrap_or we pass in an integer as the value type
    // is an integer

    // Iterating over HashMaps
    for (key,value) in &student{
        println!("Key: {}, Value: {}",key,value);
    }

    // HashMaps ownerships
    let mut student = HashMap::new();
    let branch_key = "Branch".to_string(); // doesn't implement copy trait
    let branch_val =  45;
    student.insert(branch_key,branch_val);
    println!("\n{}\n",branch_val); 
    // valid as branch_val has copy trait and the value is copied to hashmap
    // println!("{}",branch_key); fails as branch_key is moved at line 50
    // one way to solve this is to use &branch_key or branch_key.clone()
    

    // Adding a Key only if it isn't present
    student.entry(String::from("Year")).or_insert(2016);
    // checks if the key Year exists, if it doesn't then adds k:v, "Age":"22"
    // the or_insert then returns the mutable reference to the specified key

    let year = student.entry(String::from("Year")).or_insert(2016);
    // year is a mutable reference to the "value: 2016" of the "Key: Year"
    *year += 1;
    println!("Year: {}\n",year);

    student.entry(String::from("Year")).or_insert(2016);

    // Write a Program to Count the Occurnce of Each Word and Store in HashMap
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Occurence of Each Word in the sentence: {}",text);
    println!("{:?}", map);

    // split_whitespace() returns an iterator over subslices separated by whitespace
}