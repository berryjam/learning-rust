// enum List {
//     Cons(i32, List), // 编译错误，因为编译器不知道List的具体size
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Using a Box<T> to Store Data on the Heap
    let b = Box::new(5);
    println!("b = {}", b);

    // Enabling Recursive Types with Boxes 使用Box的时候，因为指针大小是确定的，指向的堆具体内容大小可以是不固定的，避免因为编译器编译时不知晓确切大小而编译出错
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Computing the Size of a Non-Recursive Type
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}