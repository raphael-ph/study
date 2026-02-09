use std::cmp::Ordering; // Ordering is an Enum
use std::io;

use rand::Rng;

fn main() {
    println!("**Guess the number**");
    println!("Please, input your guess: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        // In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. 
        let mut guess = String::new(); // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
