fn main() {
    let f = five();
    println!("{f}");

}

// rust does not care where the function is defined, before or after the main()
fn not_the_main_function(x: i32, unit_label: &str) {
    println!("The measurment is {x}{unit_label}");
}

fn statement_vs_exp() {
    let y = 6; // statement
    // let x = (let y = 6); -> Error because statements do not return values
    // example of an expression
    let y = {
        let x = 3;
        // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, 
        // you turn it into a statement, and it will then not return a value
        x + 1
    };

    println!("The value of y is: {y}")
}

// when the function returns something, we need to have the type annotation, showing the return
fn five() -> i32 {
    5
}
// --- My notes ---
// Rust makes a distinction between statements and expressions
// - Statements are instructions that perform some action and **do not** return a value.
// - Expressions evaluate to a **resultant value**.

