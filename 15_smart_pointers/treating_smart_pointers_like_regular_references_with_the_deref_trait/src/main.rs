fn main() {
    // Following the Pointer to the Value with the Dereference Operator
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using Box<T> Like a Reference
    let x = 5;
    let y = Box::new(x); // 使用时与引用无异

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 实际上Rust会执行*(y.deref())

    let m = MyBox::new(String::from("Rust"));
    hello("Rust");
    // 1.&m是MyBox<String>类型的引用，由于MyBox实现了Deref trait，Rust会调用其deref方法，对引用&MyBox<String>转为&String
    // 2.由于标准库中，String也实现了Deref trait，所以Rust会调用其deref方法，将&String转为&str
    // 3.最后&str满足了hello方法参数类型
    hello(&m);
    // 如果Rust不支持deref coercion特性，将需要这样调用hello(&(*m)[..]);
}

// Defining Our Own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Treating a Type Like a Reference by Implementing the Deref Trait 要对自定义引用类型进行解引用，需要实现Deref trait
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // type语法定义了Deref trait的关联类型

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implicit Deref Coercions with Functions and Methods
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// How Deref Coercion Interacts with Mutability （Deref对应immutable引用，DerefMut对应mutable引用）
/*
Rust does deref coercion when it finds types and trait implementations in three cases:

From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
*/