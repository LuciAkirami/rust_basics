/* 
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

To use an Iterator Trait, one must define a Type "Item", and the the fn "next" which returns an
"Option" containing that Item. Item is basically the data type on which we are iterating. Like for
a vec<i32>, the Item will be i32, as we are iterating a vector of i32 

Iterators are lazy, they do not do anything until unless they are called like the calling the
next fn

iter() yields &T – immutable references.
iter_mut() yields, as the name suggests, &mut T – mutable references.
into_iter() yields any of T (moved value), &T or &mut T depending on the usage.
*/
fn main(){
    let v1 = vec![1, 2, 3];
    // to call the next() method, we need to use "mut" as the next() takes &mut self
    // we need mut because, when we call next(), we are changing the internal state of the
    // of the Iterator, by moving it to the next value, hence "mut" is needed
    let mut v1_iter = v1.iter(); // lazy doesn't do anything

    // but in "for loop", we don't require this, it happens automatically
    // for i in v1.iter(){ // not using mut v1.iter()
    //     println!("{i}");
    // }

    // the next() method returns an Option, thus when call next() after the last
    // element it returns None, saying we've covered all elements
    // so basically the iterator method is being called here hence not lazy anymore
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // ----- Iterator Method -------
    // Consuming Adapters: These are the methods that call the next(), thus consuming the
    // Iterators. Eg: sum()

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // sum() takes away the ownership, thus v1_iter is moved
    println!("Total is {total}\n");
    // println!("{:?}",v1_iter.next()); panics as the v1_iter is already used by .sum()

    // Iterator Adaptors: These methods take in/consume the iterators and produce new iterators
    // Most Iterator Adapters take closures as arguements
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // ----- MAP METHOD -----
    // here map() takes in the Iterator and a closure and returns a new iterator
    // this new iterator is lazy, i.e doesn't do anything until called
    let changed_v1_iter = v1_iter.map(|x| x+1);

    for i in changed_v1_iter{ // not using mut v1.iter()
        println!("{i}");
    }

    // Using the Collect Method to convert Iterator to a Collection like Vector
    // We need to explicity declare the Type annotation(which collection we're using) 
    // when using Collect. And the collect fn here returns a new modified vector not related to v1
    let v1 = vec![1, 2, 3, 4];
    let v1_incr_by_1: Vec<_> = v1.iter().map(|x| x+10).collect();// reuturns vec<&i32>
    println!("\nOld Vector: {:?} \nNew Vector: {:?}\n",v1,v1_incr_by_1); 

    // FILTER METHOD - Returns a Filtered Iterator
    // v.iter() produces an Iterator<Item = &i32>. The .filter() call takes an Iterator<Item = T> as 
    // input, and passes &T to its predicate. Therefore x: &&i32 
    // To perform some operations like "%", which only supports &var and var but not &&var
    // we need to dereference the &&var by one time here collect will return Vec<&i32>, a reference to v1, 
    // because Filter returns a reference to the Iterator v1.iter(), which inter a reference to v1
    let v1 = vec![1, 2, 3, 4];
    let changed_v1_iter = v1.iter().filter(|x| *x%2 == 0);
    let v1_even: Vec<_> = changed_v1_iter.collect();
    // drop(v1); will not work as v1 is being refered by v1_even
    println!("Old Vector: {:?} \nNew Vector: {:?}\n",v1,v1_even);

    // -------- Types of Iterators ---------

    // ---- Iter Mut ----
    // Itermut Gives Mutable References to the elements in the list
    let mut v1 = vec![1, 2, 3, 4];
    println!("Old Vector: {:?}",v1);

    // updating each element by iterating through it's mutable references
    // for_each takes each element and applies the closure fn to each variable
    v1.iter_mut().for_each(|x| *x*=2);
    println!("New Vector: {:?}",v1);

    // let x:Option<&i32> = v1.iter().next();
    // let x:Option<&mut i32> = v1.iter_mut().next();
    // let x:Option<i32> = v1_iter.next();

    // let's multiply the first element by 10
    *v1.iter_mut().next().unwrap() *= 10; // .next() gets Some(&mut i32) and unwrap -> &mut i32 -> use * to dereference
    println!("New Vector again: {:?}\n",v1);

    // ---- Into Iter ----
    // This takes the ownership of the variable
    let v1 = vec![1, 2, 3, 4];
    let mut v1_iter = v1.into_iter(); // v1 is being "moved" -> ownership taken by v1_iter
    // println!("{:?}",v1); panics as the ownership of v1 is already taken by v1_ter
    // Here we see the return type is no longer a reference(Option<&i32>), but the value itself
    let x = v1_iter.next();
    println!("I'm owned, the first value is {}\n",x.unwrap());

    // ------ Some more Iterator Methods -------
    // chain(): Takes two iterators and creates a new iteraor containing both in sequence
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];

    let chained_iter = v1.iter().chain(v2.iter());

    println!("V1 {:?}\nV2 {:?}\n",v1,v2);
    println!("Looping Through Chained Iterator(V1 and V2):");

    for ele in chained_iter{
        println!("{ele}");
    }

    // using chain() to return a new Vector combining two vectors
    // the collect() fn returns a reference vec, which refers both v1 and v2
    let chained_iter = v1.iter().chain(v2.iter());
    let v1_and_v2: Vec<_> = chained_iter.collect(); // returns Vec<&i32>
    let v3 = v1_and_v2; // v3 is a reference of v1 and v2
    println!("{v3:?}\n"); // same as println!("{:?}",v3)

    // ---- last() ------
    // returns the ref to last ele and iterator is no longer usable as its consumed
    let v1 = vec![1, 2, 3, 4];
    let last = v1.last().unwrap();
    assert_eq!(*last,4);

    // ---- step_by() -> Iterate in Steps -----
    let v1 = vec![1, 2, 3, 4];
    let v1_step = v1.iter().step_by(2);
    println!("Using Stepby with Stepsize 2");
    v1_step.for_each(|x| println!("{x}"));

    // ---- zip() -----
    // takes in two iterators and returns a iterator that iterates over a tuple of ele from given two iterators

    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![6, 7, 8, 9];

    // as we can see its a type Iterator that iterates over tuples
    let zip_v1_v2 = v1.iter().zip(v2.iter());
    println!("\nZipping Two Vectors Iterators");
    zip_v1_v2.for_each(|x| println!("V1 Element: {}, V2 Element: {}",x.0,x.1));

    // the below also does th same
    // for (ele1,ele2) in zip_v1_v2{
    //     println!(println!("V1 Element: {}, V2 Element: {}",ele1,ele2));
    // }

    // ---- enumerate() ----
    // Creates an iterator which gives the current iteration count as well as the next value.
    let v1 = vec![12, 22, 32, 42];

    // as we can see its a type Iterator that iterates over tuples
    let v1_enum = v1.iter().enumerate();
    println!("\nZipping Two Vectors Iterators");
    v1_enum.for_each(|x| println!("Element Index: {}, Element: {}",x.0,x.1));

    // ---- skip(), skip_while(), take(), take_while() ----
    // skip(): Creates an iterator that skips the first n elements.
    // skip_while(): Creates an iterator that [skip]s elements based on a predicate.
    // take: Creates an iterator that only takes the first n elements.
    // take_while: Similar lo skip_while(), but instead skipping, it will take them

    let v1 = vec![12, -5, 32, 42];
    println!("\nSkip first 2 elements");
    v1.iter().skip(2).for_each(|x| println!("Did not Skip {x}"));

    // negative numbers must be followed by the type of other numbers
    let v5 = vec![12, -5i32, 32, 42];
    println!("\nSkip Negative Elements");
    #[allow(unused_mut)]
    let mut skip_negative = v5.iter().skip_while(|x| x.is_negative());
    skip_negative.for_each(|x| println!("Did not Skip {x}"));

    
    let v1 = vec![12, -5, 32, 42, 50, 96, 12, 86, 45];
    println!("\nSkip first 2 elements and from the remaining elements take only the first 3");
    // skips 12, -5 and from the remaning, takes only the first 3, i.e. 32, 42, 50
    v1.iter().skip(2).take(3).for_each(|x| println!("Did not Skip and Took {x}"));


    // --- fold() ----
    // Folds every element into an accumulator by applying an operation, returning the final result.
    let v2 = vec![(12,1), (-5,6), (32,20), (42,-2)];

    // the sum of all of the second element in each tuple in the vector
    // "acc" by default is 0, with each iteration it gets added with second element
    // and the "acc" is returned after the final itertion by the fold()
    let sum = v2.iter().fold(0, |acc, x| acc + x.1);
    println!("\nSum of all 2nd elements in tuple in the V1 {v2:?}: {sum}");

    // ---- peek() -> Look at values without advancing ---
    // let v1 = vec![12, -5, 32, 42, 50, 96, 12, 86, 45];
    // we can peek() multiple times, the iterator won't advance

    let mut iter = v1.iter().peekable();
    iter.next(); // here we advance from 12 to -5
    assert_eq!(iter.peek(), Some(&&-5)); // here its -5
    assert_eq!(iter.peek(), Some(&&-5)); // here also its -5
    assert_eq!(iter.next(), Some(&-5)); // here also its -5 and then we advance to 32

}

/*
Calling Collect Yeilds different Results Based on after what your calling it. It can return a new Collection
or a reference to an Existing Collection

Eg: With Filter, Chain
let v1 = vec![1,2];
let changed_v1: Vec<&;32> = v1.iter().filter(|x| *x%2 == 0).collect();

Here filter return "impl Iterator<Item = &i32>", a reference of the Iterator v1.iter(), which is again a 
reference to the vector v1, as the Item=&i32, the collect returns a reference vector Vec<&i32>, which references
the v1, hence changed_v1 is reference of v1

Eg: With Map
let v1 = vec![1,2];
let changed_v1: Vec<&;32> = v1.iter().map(|x| x+2 == 0).collect();

Here map returns "impl Iterator<Item = i32>", which is a completely new Iterator and not a reference to 
v1.iter(), hence calling collect() on it will return a Vec<i32>, which is a completely new vector
and not a reference to any

*/