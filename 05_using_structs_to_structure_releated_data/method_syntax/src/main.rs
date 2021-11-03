#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法的第一个参数必须为self，所以可以省略类型Self，但仍然需要使用引用进行借用而不是改变ownership，如果想改变的话需要显示声明为mut类型
    fn area(&self) -> u32 { // The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for.
        self.width * self.height
    }

    // Methods like this are called getters，结构体内声明的方法如果与字段名同名，在rust被称为getters方法，一般只返回对应的字段值
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool { // 这里使用引用进行借用，避免更改other的ownship，因为也只是读，所以不需要mut
        self.width > other.width && self.height > other.height
    }

    // Associated Functions  不带self参数的方法一般用作constructor，返回对应结构体的实例，使用上不使用.语法，而是:: ，e.g let sq = Rectangle::square(3);
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Multiple impl Blocks 分开impl的block在generic types and traits会排得上用场
impl Rectangle {
    fn can_hold_in_another_block(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}