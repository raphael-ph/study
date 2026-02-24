fn main() {
    // first_example()
    // second_example()
    // third_example()
    fourth_example()
}

fn fourth_example() {
    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("red spectre from black: {0}", black.0)
}

fn third_example() {
    // defining a struct
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    };

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // using update syntax. This way we do no thave to rewrite
    // everything and reuse the declaration of the previously
    // intantiated struct
    // ---
    // Note that the struct update syntax uses = like an assignment; this is because it moves the data, 
    // just as we saw in the “Variables and Data Interacting with Move” section. In this example, we can no longer 
    // use user1 after creating user2 because the String in the username field of user1 was moved into user2
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // comes last
    };

    println!("New user email: {0}", user2.email)
}


fn second_example() {
    // defining a struct
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user2 = build_user("user2@example.com".to_string(), "someotheruser123".to_string());
    println!("New user name: {0}", user2.username)
}

fn first_example() {
    // defining a struct
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("Is user active: {0}", user1.active)

}
