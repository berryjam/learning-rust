fn main() {
    let x = 1;

    // Matching Literals
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 这里会匹配上，y将shadow掉外层的y，被赋值为5
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // ｜ 逻辑或，任意匹配到其中一种情况即可
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=  ，范围匹配只适用于数值或char类型的对象
    let x = 5;

    match x {
        1..=5 => println!("one through five"), // 范围为[1,5]，相比｜，在条件数量比较多的时候更简洁
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring to Break Apart Values
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // 使用let分离structs
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y), // 会匹配到第一个x，后面的y是被shadow的新变量
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructuring Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    // Destructuring Nested Structs and Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // Destructuring Structs and Tuples 同时对数组和struct进行拆分匹配
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring Values in a Pattern

    // Ignoring an Entire Value with _，通过下划线忽略其他值，通过作为match pattern最后一个匹配项，起到catchall能力
    fn foo(_: i32, y: i32) { // 忽略第一个参数
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // Ignoring Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { // 下划线忽略匹配到的值
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => { // 忽略部分值
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // Ignoring an Unused Variable by Starting Its Name with _ ，通过前缀下划线，忽略unused变量警告
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s { // _s仍然会绑定变量，导致s的ownership被修改
        println!("found a string");
    }

    // println!("{:?}", s); 编译错误，_s已经修改s的ownership，后续无法继续使用

    let s = Some(String::from("Hello!"));

    if let Some(_) = s { // 单独下划线不会绑定任何变量
        println!("found a string");
    }

    println!("{:?}", s);

    // Ignoring Remaining Parts of a Value with .. ，通过..直接忽略剩余部分，避免用多个_分别去忽略
    struct Point1 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point1 { x: 0, y: 0, z: 0 };

    match origin {
        Point1 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => { // ..会根据实际要忽略的数量而展开,但是某些场景下存在歧义，会产生编译错误
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let numbers = (2, 4, 8, 16, 32);

    // match numbers {
    //     (.., second, ..) => { // ..存在编译，编译错误
    //         println!("Some numbers: {}", second)
    //     }
    // }

    /*
    Extra Conditionals with Match Guards
    在match arm后续追加的if语句即match guard，进一步保证匹配的条件
    这能够让match arm有能力限定匹配值的额外条件
    */
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // match guard与|组合,等价于(4 | 5 | 6) if y
        _ => println!("no"),
    }

    // @ Bindings
    enum Message1 {
        Hello { id: i32 },
    }

    let msg = Message1::Hello { id: 5 };

    match msg {
        Message1::Hello {
            id: id_variable @ 3..=7, // @绑定，既能测试变量是否在范围内，而且还能绑定到新变量，后续可以直接使用
        } => println!("Found an id in range: {}", id_variable),
        Message1::Hello { id: 10..=12 } => { // 这里相比@，后续无法使用匹配到的值
            println!("Found an id in another range")
        }
        // 最后一个match arm，相比@绑定，能赋值到所匹配的值到id，但又不能匹配范围
        Message1::Hello { id } => println!("Found some other id: {}", id),
    }
}
