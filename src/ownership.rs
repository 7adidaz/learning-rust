fn main() {
    let s: String = String::from("hello");
    takes_onwership(s); // moves s the the function
                        // println!("{}", s); // breaks!

    let mut x: i32 = 0;
    makes_copy(x);
    x += 2;
    // prints 2.. because x inside the function is COPIED!
    println!("{} after the clone", x);

    let s1: String = gives_ownership();
    println!("s1 is now hava another owner! ->  {}", s1);

    let mut s2: String = String::from("Hi, from the first Owner! ");
    s2 = takes_and_gives_back(s2); // takes the ownership back
    println!("{}", s2);

    let s3: String = String::from("hello");
    let len = takes_value_as_refrence_and_calculate_length(&s3);
    println!("{}", len);

    // mutable refrences have restrictions
    let mut var = String::from("hi in main");

    let ref1 = &mut var; //only mutable refrence for a variable per scope
                         // let ref2 = &mut var; // this fails

    println!("{}", ref1 /* , ref2*/);
}

fn takes_onwership(some_string: String) {
    println!("{} inside a function", some_string);
    // after the function is poped off the stack, 'some_string' get dropped!
}

fn makes_copy(mut some_int: i32) {
    some_int += 10;
    println!("{} int is inside a function", some_int); // this is acually copied!
}

fn gives_ownership() -> String {
    let some: String = String::from("hello, i'm owned by a function");
    some
}

fn takes_and_gives_back(mut a_string: String) -> String {
    a_string += ".. HI from the other owner";
    a_string
}

fn takes_value_as_refrence_and_calculate_length(s: &String) -> usize {
    s.len()
}
