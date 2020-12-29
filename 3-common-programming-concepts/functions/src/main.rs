fn main() {
    println!("Hello, world!");

    another_function();
    another_function_single_param(5);
    another_function_two_params(1, 2);

    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_single_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_two_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}