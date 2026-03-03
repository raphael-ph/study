
// using match for control flow
fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    impl Coin {
        fn value_in_cents(&self) -> u8 {
            match self {
                Coin::Penny => {
                    println!("Lucky penny");
                    1
                },
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }
    println!("A Penny is worth {} cent", Coin::Penny.value_in_cents());
}

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
