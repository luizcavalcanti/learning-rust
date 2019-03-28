fn main() {
    conditionals();
    conditional_assignment();
    loop_with_returning_break();
    while_loop();
    loop_through_collection();
    loop_through_range();
}

fn conditionals() {
    let number = 5;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn conditional_assignment() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

// fn infinite_loop() {
//     loop {
//         println!("again!");
//     }
// }

fn loop_with_returning_break() {
    let mut counter = 5;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Returning loop result: {}", result);
}

fn while_loop() {
    let mut x = 5;

    while x > 0 {
        println!("while loop: {}", x);
        x -= 1;
    }
}

fn loop_through_collection() {
    let a = [10, 20, 30, 40, 50];
    for n in a.iter() {
        println!("iterating: {}", n);
    }
}

fn loop_through_range() {
    for number in 1..4 {
        println!("increasing! {}", number);
    }

    for number in (1..4).rev() {
        println!("decreasing! {}", number)
    }
}
