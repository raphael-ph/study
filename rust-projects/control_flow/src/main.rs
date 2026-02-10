fn main() {
    // first_example()
    // second_example()
    // third_example()
    // fourth_example()
    // fith_example()
    // sixth_example()
    // seventh_example()
    // eight_example()
    ninth_example()
}

fn ninth_example() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}

fn eight_example() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    for element in a {
        println!("The {index} number is {element}");
        index += 1;
    }
}

fn seventh_example() {
    // this is not a good example because the code could panic;
    // index keeps running and ACTUALLY hits 6 but we do not try to fetch
    // the value from the array. eigth_example() fix this
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The {index} number is {}", a[index]);
        index += 1;
    }

}

fn sixth_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn fith_example() {
    let mut d_count = 10; // descending count
    let mut a_count = 0;  // ascending count

    'counting_down: loop {
        println!("Down count is: {d_count}");
        'counting_up: loop {
            println!("Up count is: {a_count}");
            d_count -= 1;
            a_count += 1;
            if d_count == 3 {
                break 'counting_down;
            }
        }
    }
    println!("Final d_count: {d_count}");
    println!("Final a_count: {a_count}");

}


fn fourth_example() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
    };

    println!("Result is {}", result)
}

fn third_example() {
    loop {
        println!("Again!")
    }
}

fn second_example() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number is {number}");
}

fn first_example() {
    let number = 5;
    if number < 5 {
        println!("Lower than 5!");
    } else if number == 5 {
        println!("Exactly 5!");
    } else {
        println!("Bigger than 5!");
    }
}


