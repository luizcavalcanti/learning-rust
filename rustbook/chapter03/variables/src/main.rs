fn main() {
    println!("mutability: ");
    mutability();
    println!("shadowing: ");
    shadowing();
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}
