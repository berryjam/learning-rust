use std::io;

fn main() {
    // Data Types
    /*
    In cases when many types are possible,
    such as when we converted a String to a numeric type using parse in the “Comparing the Guess to the Secret Number” section in Chapter 2,
    we must add a type annotation, like this:
     */
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types:Integer Types
    let decimal_num = 98_222; // Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.
    let hex_num = 0xff;
    let octal_num = 0o77;
    let binary_num = 0b1111_0000;
    let byte_num = b'A';

    // Scalar Types:Floating-Point Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Scalar Types:Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // The Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound Types:The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // In addition to destructuring through pattern matching,
    // we can access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    // For example:
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // The Array Type.Unlike a tuple, every element of an array must have the same type.
    // 数组在栈上分配，而vector是在堆上分配；数组长度固定
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // i32:表示类型，5表示数组长度，声明时用；隔开
    let a = [3; 5]; // 初始化长度为5的数组，所有元素都为3

    // Accessing Array Elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
