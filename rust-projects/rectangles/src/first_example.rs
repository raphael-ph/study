#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, 
        // the width field will get the same value as if we didn’t have the dbg! call there.
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// #[derive(Debug)] // if not here, compile error!!!! -> `Rectangle` cannot be formatted using `{:?}` because it doesn't implement `Debug`
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     // println!("rect1 is {rect1}"); // compile error!!!! -> `Rectangle` cannot be formatted with the default formatter
//     println!("rect1 is {rect1:#?}");  
// }

// // refactorinbg with structs
// struct Rectangle {
//     width: u32, 
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

// }

// fn area(rectangle: &Rectangle) -> u32 { // is better for functions to borrow than to own
//     rectangle.width * rectangle.height
// }

// // using tuple to make it more readable
// fn main() {
//     let rect1: (u32, u32) = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
    
// fn area(dimensions: (u32, u32)) -> u32 {
//         dimensions.0 * dimensions.1
//     }


// fn main() {
//     let width1: u32 = 30;
//     let height1: u32 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
