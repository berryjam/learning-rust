fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_params(20);
    print_labeled_measurement(5, 'h');

    // Function Bodies Contain Statements and Expressions
    let x = 5;

    // The block that we use to create new scopes, {}, is an expression
    let y = {
        let x = 3;
        x + 1  // rust的expression不包括分号，有返回值；加上分号就变成statement，没有返回值，表达式可以用于赋值，最后一行为返回值
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_params(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Functions with Return Values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 没有分号，是一个expression
}