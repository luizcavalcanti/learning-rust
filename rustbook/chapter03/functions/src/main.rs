fn main() {
    println!("Main function");
    another_function(5, 6);
    println!("Return of function call: {}", five());
    println!("Return of another function call: {}", plus_one(five()));
}

fn another_function(x: i32, y: i32) {
    println!("Another function, params: {}, {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
