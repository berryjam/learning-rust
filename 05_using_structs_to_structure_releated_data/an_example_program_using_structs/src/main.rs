#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Refactoring with Tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_refactor(rect1)
    );

    // Refactoring with Structs: Adding More Meaning

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_refactor_more_readable(&rect1)
    );

    // Adding Useful Functionality with Derived Traits
    println!("rect1 is {:?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); // dgb宏相比println宏，会打印出源码文件名和行号
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_refactor(dimensions: (u32, u32)) -> u32 { // 相比area方法，tuple比两个分离的参数更高内聚
    dimensions.0 * dimensions.1
}

fn area_refactor_more_readable(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}