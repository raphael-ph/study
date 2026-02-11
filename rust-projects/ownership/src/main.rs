mod references_borrowing;

fn main() {
    // first_example()
    // second_example()
    // third_example()
    // fourth_example()
    // fith_example()
    // sixth_example()
    // seventh_example()
    // eight_example()
    // ninth_example()
    // tenth_example()
    eleventh_example()
}

fn eleventh_example() {
    let reference_to_nothing = references_borrowing::dangle();
}

fn tenth_example() {
    references_borrowing::combining_mutability();
}

fn ninth_example() {
    references_borrowing::mutable_reference_example();
}

// compile time error because tries to change a reference
// fn eight_example() {
//     references_borrowing::changing_references();
// }

fn seventh_example() {
    references_borrowing::bad_example();
    references_borrowing::good_example();
}

// --------------------------------------------------------------------

fn sixth_example() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

// --------------------------------------------------------------------

fn fith_example() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // println!("{s}, world!");     // this will raise compile error!

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("OK to use x = {x}")   // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// --------------------------------------------------------------------

fn fourth_example() { 
    // The reason is that types such as integers that have a known size at compile time are stored entirely 
    // on the stack, so copies of the actual values are quick to make. That means there’s no reason we would 
    // want to prevent x from being valid after we create the variable y. In other words, there’s no difference 
    // between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow 
    // copying, and we can leave it out.
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

}

fn third_example() {
    // making deep copies;
    // This here is a heap copy and actually a very expensive way of programming, meaning that
    // we are copying the value again to the heap, which is very bad.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

}

fn second_example() {
    let mut s = String::from("hello"); // this value is dropped right away.
    s = String::from("ahoy");

    println!("{s}, world!");
}

// fn first_example() {
//     // this first example raises a compile error;
//     // This happens because the value has moved (like a shallow copy) from s1 to s2.
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}, world!");
// }