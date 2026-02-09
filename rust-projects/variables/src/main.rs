fn main() {
    let tup: (i32, f64, i32) = (500, 6.4, 1);

    let (y, _x, _z): (i32, f64, i32) = tup; // desturcturing

    println!("The value of y is: {y}");
    println!("The value of x is: {} and the value of y is {}", tup.1, tup.0);

    // arrays - number of elements does not change
    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{}", a[0]);

    println!("==== Mathematical Operations ====");
    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("{}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    println!("==== Booleans ====");
    let t: bool = true;
    let f: bool = false;
    if t==f  {
        println!("They are equal");
    }
    else {
        println!("They are different!");
    }

    // initializing arrays in an interesting way
    let _a= [3; 5];

}