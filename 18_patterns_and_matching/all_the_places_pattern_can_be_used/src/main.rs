fn main() {
    /*
    match Arms

    match唯一要求，就是对类型的所有情况都需要覆盖到，_能匹配所有情况
    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }
     */

    // Conditional if let Expressions ，if let只匹配一种情况，相比match缺点就是编译器无法确保所有情况都被考虑和处理
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { // Ok(age)里面的age会shadowed右值age
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let Conditional Loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 与if let类似，只不过是与while循环结合，能够连续执行多次
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for Loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let Statements，实际是也是一种pattern，形式化为let PATTERN = EXPRESSION;
    let x = 5;
    let (x, y, z) = (1, 2, 3);

    // let (x, y) = (1, 2, 3); 编译错误，pattern和数组元素数量对不上
}

// Function Parameters
fn foo(x: i32) { // 函数参数也是一种pattern，等同于let x = xxx，其中xx为i32类型
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}