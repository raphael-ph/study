// Methods are similar to functions: We declare them with the fn keyword and a name, they can have parameters and a return value, 
// and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct 
// (or an enum or a trait object, which we cover in Chapter 6 and Chapter 18, respectively), and their first parameter is always self, 
// which represents the instance of the struct the method is being called on.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // impl = implementation
    // Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2*self.width + 2*self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 5,
        height: 30,
    };

    let sq = Rectangle::square(3); // To call this associated function, we use the :: syntax with the struct name

    println!("Our square here: {sq:#?}");

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!(
        "The perimeter of the rectangle is {} pixels",
        rect1.perimeter()
    );

    if rect1.can_hold(&rect2) {
        println!("Rectangle 1 can hold Rectangle 2");
    } else {
        println!("Rectangle 2 cannot be held by Rectangle 1");
    }

    if rect1.width() && rect1.width != 10 {
        println!(
            "The rectangle has a nonzero width. The value is: {}",
            rect1.width,
        );
    } else if rect1.width == 10 {
        println!("The rectangle width is equal to 10.");

    } else {
        println!("The rectangle cannot exist!");
    }
}