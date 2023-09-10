use std::collections::HashMap;
fn main(){
    /*
        Question: Given a vector find the median and mode
    */
    let mut vec = vec![12,54,124,13,5,2,5,11,5,23,12,9];
    let vec_len = vec.len();
    println!("Vector Array is {:?}",vec);

    // sorting the vector
    vec.sort();
    println!("Sorted Vector Array: {:?}",vec);

    // finding the median;
    // even_number "bitwise and" with 1 gives 0, and odd_number gives 1
    if (vec_len & 1) == 0{ // if true imples length is even
        
        let prev_val = vec[(vec_len/2)-1] as f32; // 6th ele
        let next_val = vec[vec_len/2] as f32; // 7th ele

        let median = (prev_val + next_val) / 2.;

        println!("Median is {median}");
    } else { // length is odd
        let median = vec[vec_len/2];
        println!("Median is {median}");
    }

    // Finding Mode;
    let mut num_occurence = HashMap::new();
    let mut best_count = 0;
    let mut mode = 0;
    for num in vec.iter(){
        let count = num_occurence.entry(num).or_insert(0);
        *count+=1;

        if *count > best_count{
            mode = *num;
            best_count = *count;
        }
    }

    println!("Mode is {mode}");

}