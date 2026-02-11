pub fn dangle() -> &String { // dangle returns a reference to a String
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!

pub fn combining_mutability() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

}

pub fn mutable_reference_example() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("The value of s is {s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// pub fn changing_references() {
//     // Just as variables are immutable by default, so are references. We’re not allowed to modify 
//     // something we have a reference to.
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

//# GOOD EXAMPLE
pub fn good_example() {
    let s1 = String::from("hello");
    let len = good_calculate_length(&s1); // reference is like a pointer but we guarantee there is valid data on it.

    println!("The length of '{s1}' is {len}.");
}

fn good_calculate_length(s: &String) -> usize { // we pass a reference
    // When functions have references as parameters instead of the actual values, we won’t need to return the values 
    // in order to give back ownership, because we never had ownership.
    s.len()
}

//# BAD EXAMPLE
pub fn bad_example() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // since we moved s1 to calculate_length, we have to moe it again.
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}