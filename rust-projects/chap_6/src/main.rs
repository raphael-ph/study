// if ... let pattern
fn main() {
    //you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
                // -- snip --
            }
        }
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let second_coin = Coin::Penny;
    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    fn describe_state_quarter(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }

    let description: Option<String> = describe_state_quarter(coin);
    let second_description : Option<String> = describe_state_quarter(second_coin);
    println!("{}", description.unwrap());
    println!("{:?}", second_description);



}

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five: Option<i32> = Some(5);
//     let six: Option<i32> = plus_one(five);
//     let none: Option<i32> = plus_one(None);

//     println!("Five is worth {five:?}");
//     println!("Six is worth {six:?}");
//     println!("None is worth {none:?}");
// }


// using match for control flow
// fn main() {
//     #[derive(Debug)]
//     enum UsState {
//         Alabama,
//         Alaska,
//     }
//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter(UsState),
//     }

//     impl Coin {
//         fn value_in_cents(&self) -> u8 {
//             match self {
//                 Coin::Penny => {
//                     println!("Lucky penny!");
//                     1
//                 },
//                 Coin::Nickel => 5,
//                 Coin::Dime => 10,
//                 Coin::Quarter(state) => {
//                     println!("State quarter from {state:?}!");
//                     25
//                 },
//             }
//         }
//     }
//     println!("A Quarter is worth {} cent", Coin::Quarter(UsState::Alabama).value_in_cents());
// }

// The Option Enum
// fn main() {
//     // The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a different type. 
//     // Rust can infer these types because we’ve specified a value inside the Some variant. For absent_number, 
//     // Rust requires us to annotate the overall Option type: The compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value. 
//     // Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
//     // let some_number = Some(5);
//     // let some_char = Some('e');

//     // let absent_number: Option<i32> = None;

//     let a: Option<i8> = Some(2);
//     let b: Option<i8> = Some(4);

//     // let sum = a + b; // compile error -> Cannot perform operations with Option<T>. Got to convert to T.
// }

// new example with enum and different types
// fn main() {
//     #[derive(Debug)]
//     enum Message { 
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }

//     // we can define methods on enums
//     impl Message {
//         fn call(&self) {
//             println!("Random message!")
//         }
//     }
    
//     let m = Message::Write(String::from("Hello"));
//     println!("{:?}", m);
//     m.call();
// }

// representing everythig as an enum. In rust, enums can hold values;
// fn main() {
//     // The name of each enum variant that we define also becomes a function that constructs an instance of the enum. 
//     // That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type. 
//     // We automatically get this constructor function defined as a result of defining the enum.
//     #[derive(Debug)]
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));

//     println!("Home's IP kind is: {:#?}", home.V4)
// }

// fn main() {
//     #[derive(Debug)]
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

//     println!("Home's IP kind is: {:#?}", home.kind)
// }
