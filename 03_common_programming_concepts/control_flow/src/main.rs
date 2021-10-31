fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Handling Multiple Conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // if也是一个expression,所以可以用于赋值，但是if分支和else分支的返回值类型必须匹配
    // let number = if condition { 5 } else { "six" }; 编译错误，if分支和else分支返回值类型不匹配，rust需要在编译时就知道对象类型

    println!("The value of number is: {}", number);

    // Repetition with Loops：loop, while, and for
    // loop {
    //     println!("again!");
    // }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // Returning Values from Loops
    let mut counter = 0;

    let result = loop { // loop也可以用于返回值，需要把返回值放到break后面
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // 不推荐下面用法，容易导致运行时异常，比如index>=5的时候，而且运行效率较低，编译器会增加检查数组下标检测的运行时代码
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // 推荐下面这种遍历方式
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() { // （1..4）是Range类型，左闭右开，由标准库提供
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
